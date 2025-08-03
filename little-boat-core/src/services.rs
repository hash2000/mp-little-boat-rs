mod signaling;

use little_boat_abstractions::{ControlEvent, IConfigReader, ServiceEvent};
use std::collections::HashMap;
use tokio::sync::{broadcast, mpsc};

use crate::{config::Config, services::signaling::start_signaling_service};

struct ServiceHandle {
  handle: tokio::task::JoinHandle<()>,
}

pub struct ServiceManager {
  service_tx: mpsc::UnboundedSender<ServiceEvent>,
  control_tx: broadcast::Sender<ControlEvent>,
  services: HashMap<String, ServiceHandle>,
}

impl ServiceManager {
  pub fn new(cfg: &dyn IConfigReader) -> Self {
    let channel_capacity = cfg.get_int(b"service-manager.broadcast.channel.capacity", 100);

    let (service_tx, _) = mpsc::unbounded_channel();
    let (control_tx, _) = broadcast::channel(channel_capacity);

    let manager =
      ServiceManager { service_tx, control_tx: control_tx.clone(), services: HashMap::new() };

    manager
  }

  pub fn start(&mut self, name: &str, cfg: &Config) {
    if self.services.contains_key(name) {
      // allready started
      return;
    }

    let service_tx = self.service_tx.clone();
    let control_tx = self.control_tx.subscribe();

    let handle = if name == "signaling" {
      start_signaling_service(service_tx, control_tx, cfg)
    } else {
      return;
    };

    self.services.insert(name.to_string(), ServiceHandle { handle });
  }

  pub fn stop(&self, name: &str) {
    let _ = self.control_tx.send(ControlEvent::Stop(name.to_string()));
  }

  pub fn shutdown(&self) {
    let _ = self.control_tx.send(ControlEvent::Shutdown);
  }

}
