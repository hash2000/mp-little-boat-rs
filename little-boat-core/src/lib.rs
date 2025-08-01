mod config;

use crate::config::{Config, init_config};

pub async fn run_client_app() -> anyhow::Result<()> {
  let mut cfg = Config::new("common")?;
  init_config(&mut cfg);

  Ok(())
}
