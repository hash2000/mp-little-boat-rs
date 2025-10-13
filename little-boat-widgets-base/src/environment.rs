use std::env;
use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "config.toml";
pub const APP_NAME: &str = "mp_little_boat";

pub fn config_dir() -> PathBuf {
  portable_dir().unwrap_or_else(platform_specific_config_dir)
}

pub fn data_dir() -> PathBuf {
  portable_dir()
    .unwrap_or_else(|| directories_next::BaseDirs::new()
      .expect("expected valid data dir")
      .data_dir()
      .join(APP_NAME))
}

pub fn cache_dir() -> PathBuf {
  directories_next::BaseDirs::new()
    .expect("expected valid cache dir")
    .cache_dir()
    .join(APP_NAME)
}

/// Checks if a config file exists in the same directory as the executable.
/// If so, it'll use that directory for both config & data dirs.
fn portable_dir() -> Option<PathBuf> {
  let exe = env::current_exe().ok()?;
  let dir = exe.parent()?;

  dir.join(CONFIG_FILE_NAME).is_file().then(|| dir.to_path_buf())
}

fn platform_specific_config_dir() -> PathBuf {
  #[cfg(target_os = "macos")]
  {
    // Priority order for config directory on macOS:
    // 1. XDG config dir (~/.config/mp_little_boat)
    // 2. User config directory (~/Library/Application Support/mp_little_boat)
    xdg_config_dir()
      .unwrap_or_else(|| directories_next::BaseDirs::new()
        .expect("expected valid config dir")
        .config_dir()
        .join(APP_NAME)
      )
  }
  #[cfg(not(target_os = "macos"))]
  {
    directories_next::BaseDirs::new()
      .expect("expected valid config dir")
      .config_dir()
      .join(APP_NAME)
  }
}
