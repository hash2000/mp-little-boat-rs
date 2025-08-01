mod defaults;

use directories_next::ProjectDirs;
use little_boat_db::database::Database;
use std::path::PathBuf;

pub use defaults::init_config;

pub struct Config {
  project_dir: ProjectDirs,
  db: Database,
  conf_path: PathBuf,
}

impl Config {
  pub fn new(name: &str) -> anyhow::Result<Self> {
    let project_dir = ProjectDirs::from("ru", "mp", "little-boat")
      .ok_or(anyhow::anyhow!("Can't open project directory"))?;

    let conf_path = project_dir.config_dir().to_path_buf();

    let db = Database::new(&conf_path, name, None)?;

    Ok(Self {
      project_dir,
      db,
      conf_path,
    })
  }

  pub fn fresh(&mut self, clear_flag: bool) -> bool {
    self.db.fresh(clear_flag)
  }

  pub fn get_config_path(&self) -> PathBuf {
    self.conf_path.clone()
  }

  pub fn has_flag(&self, key: &[u8], def: bool) -> bool {
    self.db.contains(key).unwrap_or(def)
  }

  pub fn set_flag(&self, key: &[u8]) {
    let _ = self.db.set(key, &[0]);
  }

  pub fn get_str(&self, key: &[u8], def: &str) -> String {
    self.db.get(key).map_or(def.to_string(), |v| {
      v.map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .unwrap_or(def.to_string())
    })
  }

  pub fn set_str(&self, key: &[u8], value: &str) -> anyhow::Result<()> {
    self.db.set(key, value.as_bytes())
  }

  pub fn get_json(&self, key: &[u8]) -> Option<simd_json::OwnedValue> {
    self.db.get_json(key).ok()?
  }

  pub fn set_json(&self, key: &[u8], value: simd_json::OwnedValue) -> anyhow::Result<()> {
    self.db.set_json(key, value)
  }

  pub fn get_bool(&self, key: &[u8], def: bool) -> bool {
    self.db.get(key).map_or(def, |v| {
      v
        .and_then(|bytes| bytes.first().copied())
        .map(|byte| byte == 1)
        .unwrap_or(def)
    })
  }

  pub fn set_bool(&self, key: &[u8], value: bool) -> anyhow::Result<()> {
    self.db.set(key, if value { &[1] } else { &[0] })
  }
  
  pub fn get_float(&self, key: &[u8], def: f64) -> f64 {
      match self.db.get(key) {
          Ok(Some(bytes)) if bytes.len() == 8 => {
              bytes.try_into()
                  .map(f64::from_le_bytes)
                  .unwrap_or(def)
          },
          _ => def,
      }
  }

  pub fn set_float(&self, key: &[u8], value: f64) -> anyhow::Result<()> {
      self.db.set(key, &value.to_le_bytes())
  }

  pub fn get_int(&self, key: &[u8], def: i64) -> i64 {
      match self.db.get(key) {
          Ok(Some(bytes)) if bytes.len() == 8 => {
              bytes.try_into()
                  .map(i64::from_le_bytes)
                  .unwrap_or(def)
          },
          _ => def,
      }
  }

  pub fn set_int(&self, key: &[u8], value: i64) -> anyhow::Result<()> {
      self.db.set(key, &value.to_le_bytes())
  }

}
