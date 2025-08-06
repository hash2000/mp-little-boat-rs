mod config;
mod services;

use std::{any, sync::Arc};
use little_boat_abstractions::IConfig;

use crate::{config::Config, services::ServiceManager};

pub struct ClientApp {
  // local database with all configurations
  cfg: Arc<dyn IConfig>,

  //
  service_manager: ServiceManager,
}

impl ClientApp {
  pub fn new() -> anyhow::Result<Self> {
    let cfg: Arc<dyn IConfig> = Arc::new(Config::new("common")?);
    let service_manager = ServiceManager::new(cfg.clone());

    let app = Self { 
      cfg, 
      service_manager
    };

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
    self.service_manager.wait_services().await?;
    Ok(())
  }
}
