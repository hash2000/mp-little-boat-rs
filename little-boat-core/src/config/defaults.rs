use crate::config::Config;

pub fn init_config(cfg: &mut Config) {
  if !cfg.fresh(true) {
    return;
  }
}
