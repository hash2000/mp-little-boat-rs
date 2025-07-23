mod encryption;
mod aes;
mod chacha20poly1305;

use crate::database::encryption::{Encryptor, NoOpEncryptor};
use anyhow::{Context, Result};
use simd_json::base::ValueAsObject;
use simd_json::derived::ValueObjectAccess;
use simd_json::{json, to_vec};
use sled::Db;
use std::fs::{self};
use std::path::Path;
use std::sync::Arc;

pub struct Database {
  handler: Db,
  encryptor: Arc<dyn Encryptor>,
  version: String,
  name: String,
}

fn is_sled_db_initialized(path: &Path) -> bool {
  if !path.exists() || !path.is_dir() {
    return false;
  }

  // Sled обычно создаёт файлы с такими расширениями
  let sled_extensions = ["db", "log", "blob", "conf"];

  match fs::read_dir(path) {
    Ok(entries) => entries.filter_map(|e| e.ok()).any(|entry| {
      let path = entry.path();
      if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        sled_extensions.contains(&ext)
      } else {
        false
      }
    }),
    Err(_) => false,
  }
}

impl Database {
  pub fn new(path: &Path, name: &str, encryptor: Option<Arc<dyn Encryptor>>) -> Result<Self> {
    let is_exists = is_sled_db_initialized(path);
    let db = sled::open(path)?;
    let encryptor = encryptor.unwrap_or_else(|| Arc::new(NoOpEncryptor));

    let db = Self {
      handler: db,
      encryptor,
      version: String::from("0.0.1"),
      name: name.to_string(),
    };

    if !is_exists {
      db.append_metadata(name)?;
    } else {
      db.read_metadata(name)?;
    }

    Ok(db)
  }
  
  pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
    match self.handler.get(key) {
      Ok(None) => Ok(None),
      Ok(Some(value)) => {
        self.encryptor.encrypt(&value).map(Some)
      },
      Err(e) => Err(e).with_context(|| format!("Failed to read key: {:?} database: {:?}", key, self.name)),
    }
  }
  
  pub fn get_json(&self, key: &[u8]) -> Result<Option<simd_json::OwnedValue>> {
    match self.get(key)? {
      None => Ok(None),
      Some(mut value) => {
        let json_value = simd_json::to_owned_value(&mut value)
          .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;
        Ok(Some(json_value))
      }
    }
  }

  pub fn set(&self, key: &[u8], value: &[u8]) -> Result<()> {
    let value = self.encryptor.encrypt(value)?;
    match self.handler.insert(key, value) {
      Ok(_) => Ok(()),
      Err(e) => Err(e).with_context(|| format!("Failed to set key: {:?} database: {:?}", key, self.name)),
    }
  }

  pub fn set_json(&self, key: &[u8], value: simd_json::OwnedValue) -> Result<()> {
    let value = to_vec(&value)?;
    self.set(key, &value)
  }

  pub fn contains(&self, key: &[u8]) -> Result<bool> {
    match self.handler.contains_key(key) {
      Ok(new_value) => Ok(new_value),
      Err(e) => Err(e).with_context(|| format!("Failed to read key status {:?} database: {:?}", key, self.name))
    }
  }
}

/// implements metadata methods
impl Database {

  pub fn get_version(&self) -> Result<String> {
    Ok(self.version.clone())
  }

  fn append_metadata(&self, name: &str) -> Result<()> {
    let metadata = json!({
      "database": {
        "name": name,
        "version": "0.0.1"
      }
    });

    let metadata = to_vec(&metadata)?;
    self
      .handler
      .insert(b"metadata", metadata)?
      .ok_or(anyhow::anyhow!("Can't create metadata for database [{:?}]", name))?;

    Ok(())
  }

  fn read_metadata(&self, name: &str) -> Result<()> {
    let mut metadata = self
      .handler
      .get(b"metadata")?
      .ok_or(anyhow::anyhow!("Can't read metadata from database [{:?}]", name))?;

    let metadata = simd_json::to_owned_value(&mut metadata)?;
    let db_field = match metadata.get("database") {
      None => Err(anyhow::anyhow!("Invalid metadata for database [{:?}] field [database]", name)),
      Some(value) => Ok(value),
    }?;

    if let Some(_) = db_field.as_object() {
      todo!("any checks of database metadata value");
    } else {
      return Err(anyhow::anyhow!("Invalid metadata for database [{:?}] field [database]", name));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::tempdir;

  #[test]
  fn tdd_new_database_with_metadata() {
    let dir = tempdir().unwrap();
    let _ = Database::new(dir.path(), "settings", None).unwrap();
  }
}
