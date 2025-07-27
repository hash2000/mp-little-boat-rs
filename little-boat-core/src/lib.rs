mod config;

use crate::config::{init_config, Config};

pub async fn run_client_app() -> anyhow::Result<()> {
  let mut cfg = Config::new("common")?;
  init_config(&mut cfg);


  #[cfg(feature = "console-ui")]
  little_boat_client_term::run()?;

  Ok(())
}
