mod metadata;
mod encryption;
mod aes;
mod chacha20poly1305;

use crate::database::encryption::{Encryptor, NoOpEncryptor};
use crate::database::metadata::initialize;
use anyhow::{Context, Result};
use simd_json::to_vec;
use sled::Db;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct Database {
  handler: Db,
  encryptor: Arc<dyn Encryptor>,
  version: String,
  name: String,
  path: PathBuf,
}

impl Database {
  pub fn new(path: &Path, name: &str, encryptor: Option<Arc<dyn Encryptor>>) -> Result<Self> {
    let mut path = path.to_path_buf();
    path.push(name);

    let db = sled::open(&path)?;
    let encryptor = encryptor.unwrap_or_else(|| Arc::new(NoOpEncryptor));

    let db = Self {
      handler: db,
      encryptor,
      version: String::from("0.0.1"),
      name: name.to_string(),
      path: path.to_path_buf(),
    };

    initialize(&db, name)?;

    Ok(db)
  }

  pub fn path(&self) -> PathBuf {
    self.path.clone()
  }

  pub fn exists(&self) -> bool {
    true
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
