use little_boat_abstractions::{ChatEvent, IConfig, ServiceEvent, SignalingEvent, SystemEvent};
use tokio::sync::broadcast;
use std::sync::Arc;

use crate::{config::Config, services::ServiceManager};

pub struct ClientApp {
  // local database with all configurations
  cfg: Arc<dyn IConfig>,

  // service manager
  service_manager: ServiceManager,

  event_rx: broadcast::Receiver<ServiceEvent>,
}

impl ClientApp {
  pub fn new() -> anyhow::Result<Self> {
    let cfg: Arc<dyn IConfig> = Arc::new(Config::new("common")?);
    let service_manager = ServiceManager::new(cfg.clone());
    let event_rx = service_manager.service_events();
    
    let app = Self { cfg, service_manager, event_rx };

    Ok(app)
  }

  pub async fn serve(&mut self, name: &str) -> anyhow::Result<()> {
    self.service_manager.start(name, self.cfg.clone()).await
  }

  pub fn stop(&self, name: &str) -> anyhow::Result<()> {
    self.service_manager.stop(name)
  }

  pub async fn shutdown(&mut self) -> anyhow::Result<()> {
    self.service_manager.shutdown()?;
    Ok(())
  }

  pub async fn run(&mut self) -> anyhow::Result<()> {
    self.init_logging();

    little_boat_abstractions::log_info!("client-app", "Starting client application");

    // Start serveices
    self.serve("signaling").await?;
    self.serve("chat").await?;

    // process services events
    self.process_service_events().await?;

    // wait when all services ended
    self.service_manager.wait_services().await?;

    little_boat_abstractions::log_info!("client-app", "Client application stopped");
    Ok(())
  }

  async fn process_service_events(&mut self) -> anyhow::Result<()> {
    loop {
      tokio::select! {
          // services events
          event_result = self.event_rx.recv() => {
              match event_result {
                  Ok(event) => {
                      self.handle_service_event(event).await?;
                  }
                  Err(broadcast::error::RecvError::Closed) => {
                      little_boat_abstractions::log_info!("client-app", "Service event channel closed");
                      break;
                  }
                  Err(broadcast::error::RecvError::Lagged(_)) => {
                      little_boat_abstractions::log_warn!("client-app", "Service event channel lagged");
                  }
              }
          }

          // application exit (Ctrl+C)
          _ = tokio::signal::ctrl_c() => {
              little_boat_abstractions::log_info!("client-app", "Received Ctrl+C, shutting down");
              self.shutdown().await?;
              break;
          }
      }
    }

    Ok(())
  }

  async fn handle_service_event(&self, event: ServiceEvent) -> anyhow::Result<()> {
    match event {
      ServiceEvent::Signaling(sig_event) => {
        self.handle_signaling_event(sig_event).await?;
      }
      ServiceEvent::Chat(chat_event) => {
        self.handle_chat_event(chat_event).await?;
      }
      ServiceEvent::System(sys_event) => {
        self.handle_system_event(sys_event).await?;
      }
      ServiceEvent::Status(service_event_message) => todo!(),
      ServiceEvent::Error(service_event_message) => todo!(),
    }
    Ok(())
  }

  async fn handle_signaling_event(&self, event: SignalingEvent) -> anyhow::Result<()> {
    match event {
      SignalingEvent::ClientConnected { client_id } => {
        little_boat_abstractions::log_info!("client-app", "Client connected: {}", client_id);
      }
      SignalingEvent::ClientDisconnected { client_id } => {
        little_boat_abstractions::log_info!("client-app", "Client disconnected: {}", client_id);
      }
      SignalingEvent::MessageForwarded { from, to } => {
        little_boat_abstractions::log_info!(
          "client-app",
          "Message forwarded from {} to {}",
          from,
          to
        );
      }
    }
    Ok(())
  }

  async fn handle_chat_event(&self, event: ChatEvent) -> anyhow::Result<()> {
    match event {
      ChatEvent::MessageReceived { from, content } => {
        little_boat_abstractions::log_info!("client-app", "[{}] {}", from, content);
        todo!("chat message logic here");
      }
      ChatEvent::UserJoined { user } => {
        little_boat_abstractions::log_info!("client-app", "User joined: {}", user);
      }
      ChatEvent::UserLeft { user } => {
        little_boat_abstractions::log_info!("client-app", "User left: {}", user);
      }
    }
    Ok(())
  }

  async fn handle_system_event(&self, event: SystemEvent) -> anyhow::Result<()> {
    match event {
      SystemEvent::ServiceStarted { name } => {
        little_boat_abstractions::log_info!("client-app", "Service started: {}", name);
      }
      SystemEvent::ServiceStopped { name } => {
        little_boat_abstractions::log_info!("client-app", "Service stopped: {}", name);
      }
      SystemEvent::Error { service, message } => {
        little_boat_abstractions::log_error!("client-app", "[{}] Error: {}", service, message);
      }
    }
    Ok(())
  }

  fn init_logging(&self) {
    use env_logger::Builder;
    use log::LevelFilter;

    let log_level =
      if self.cfg.get_bool(b"app.debug", false) { LevelFilter::Debug } else { LevelFilter::Info };

    // Ignore error
    Builder::new().filter(None, log_level).try_init().ok();
  }
}
