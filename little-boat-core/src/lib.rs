mod config;
mod services;

use tokio::sync::mpsc::Receiver;

use little_boat_abstractions::ControlEvent;

use crate::{
  config::Config,
  services::ServiceManager,
};


pub struct ClientApp {
  // local database with all configurations 
  cfg: Config,

  // 
  control_rx: Receiver<ControlEvent>,
}

pub async fn run_client_app() -> anyhow::Result<()> {
  let cfg = Box::new(Config::new("common")?);
  let service_manager = ServiceManager::new(cfg);
  //service_manage


  Ok(())
}
