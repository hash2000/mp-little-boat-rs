use crate::config::client::ClientConfig;
use little_boat_db::database::Database;


pub fn init_client_config(cfg: &ClientConfig) -> anyhow::Result<()> {
  let common_conf_dir = cfg.get_common_config_path();
  let common_conf_db = Database::new(&common_conf_dir, "common", None)?;


  Ok(())
}
