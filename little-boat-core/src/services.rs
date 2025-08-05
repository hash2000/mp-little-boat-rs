use little_boat_abstractions::{ControlEvent, IConfigReader, IService, ServiceEvent};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::broadcast;

struct ServiceHandle {
  handle: tokio::task::JoinHandle<anyhow::Result<()>>,
}

pub struct ServiceManager {
  service_tx: broadcast::Sender<ServiceEvent>,
  control_tx: broadcast::Sender<ControlEvent>,
  services: HashMap<String, ServiceHandle>,
  registered_services: Vec<Box<dyn IService>>,
}

impl ServiceManager {
  pub fn new(cfg: Arc<dyn IConfigReader>) -> Self {
    let channel_capacity = cfg.get_int(b"service-manager.broadcast.channel.capacity", 100);

    let (service_tx, _) = broadcast::channel(channel_capacity);
    let (control_tx, _) = broadcast::channel(channel_capacity);

    let mut manager = ServiceManager {
      service_tx,
      control_tx: control_tx.clone(),
      services: HashMap::new(),
      registered_services: Vec::new(),
    };

    manager.register(Box::new(little_boat_service_signaling::SignalingService));
    manager
  }

  pub fn register(&mut self, service: Box<dyn IService>) {
    self.registered_services.push(service);
  }

  pub async fn start(&mut self, name: &str, cfg: Arc<dyn IConfigReader>) -> anyhow::Result<()> {
    if self.services.contains_key(name) {
      little_boat_abstractions::log_info!("service-manager", "Service {} already started", name);
      return Ok(());
    }

    let service_to_start = self.registered_services.iter().find(|s| s.name() == name);

    if let Some(service) = service_to_start {
      let service_tx = self.service_tx.clone();
      let control_rx = self.control_tx.subscribe();

      let handle = service.start(service_tx, control_rx, cfg.clone()).await?;
      self.services.insert(name.to_string(), ServiceHandle { handle });

      little_boat_abstractions::log_info!("service-manager", "Started service: {}", name);
    } else {
      little_boat_abstractions::log_error!("service-manager", "Unknown service: {}", name);
      anyhow::bail!("Unknown service: {}", name);
    }

    Ok(())
  }

  pub fn stop(&self, name: &str) -> anyhow::Result<()> {
    self.control_tx.send(ControlEvent::Stop(name.to_string()))?;
    little_boat_abstractions::log_info!("service-manager", "Sent stop command for service: {}", name);
    Ok(())
  }

  pub fn shutdown(&self) -> anyhow::Result<()> {
    self.control_tx.send(ControlEvent::Shutdown)?;
    little_boat_abstractions::log_info!("service-manager", "Sent shutdown command to all services");
    Ok(())
  }

  pub fn events(&self) -> broadcast::Receiver<ServiceEvent> {
    self.service_tx.subscribe()
  }

  pub async fn wait_services(&mut self) -> anyhow::Result<()> {
    let names: Vec<String> = self.services.keys().cloned().collect();

    for name in names {
      if let Some(service) = self.services.remove(&name) {
        match service.handle.await {
          Ok(Ok(())) => {
            little_boat_abstractions::log_info!("service-manager", "Service {} completed successfully", name);
          }
          Ok(Err(e)) => {
            little_boat_abstractions::log_error!("service-manager", "Service {} failed: {}", name, e);
            return Err(e);
          }
          Err(e) => {
            little_boat_abstractions::log_error!("service-manager", "Service {} panicked: {}", name, e);
            anyhow::bail!("Service {} panicked: {}", name, e);
          }
        }
      }
    }

    Ok(())
  }
}
