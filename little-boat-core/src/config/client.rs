use std::path::PathBuf;

use directories_next::{UserDirs, ProjectDirs};

pub struct ClientConfig {
  project_dir: ProjectDirs,
  user_dirs: UserDirs,
}

impl ClientConfig {
  pub fn new() -> anyhow::Result<Self> {
    let instance = Self {
      project_dir: ProjectDirs::from("ru", "mp", "little-boat")
          .ok_or(anyhow::anyhow!("Can't open project directory"))?,
      user_dirs: UserDirs::new()
          .ok_or(anyhow::anyhow!("Can't open user directory"))?,
    };

    Ok(instance)
  }

  pub fn get_common_config_path(&self) -> PathBuf {
    let path = self.project_dir.config_dir().to_path_buf();
    path
  } 
}
