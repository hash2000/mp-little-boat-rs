mod config;

use crate::config::{init_config, Config};


pub async fn run_client_app() -> anyhow::Result<()> {
  let mut cfg = Config::new("common")?;
  init_config(&mut cfg);


  
  Ok(())
}
