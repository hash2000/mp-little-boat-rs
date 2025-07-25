mod config;

use crate::config::client::ClientConfig;
use crate::config::client_init::init_client_config;

pub async fn run_client_app() -> anyhow::Result<()> {
  let cfg = ClientConfig::new()?;
  init_client_config(&cfg)?;

  
  Ok(())
}
