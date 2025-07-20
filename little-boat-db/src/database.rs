use crate::errors::DatabaseError;
use anyhow::{Context, Error, Result};
use simd_json::base::ValueAsObject;
use simd_json::derived::ValueObjectAccess;
use simd_json::{json, to_string, to_vec, value};
use sled::{Db, IVec};
use std::f32::consts::E;
use std::fs::{self, Metadata};
use std::path::Path;

pub struct Database {
  handler: Db,
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
  pub fn new(path: &Path, name: &str) -> Result<Self> {
    let is_exists = is_sled_db_initialized(path);
    let db = sled::open(path)?;
    let db = Self {
      handler: db,
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
      Ok(Some(value)) => Ok(Some(value.to_vec())),
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

  pub fn set(&self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>> {
    match self.handler.insert(key, value) {
      Ok(None) => Ok(None),
      Ok(Some(new_value)) => Ok(Some(new_value.to_vec())),
      Err(e) => Err(e).with_context(|| format!("Failed to set key: {:?} database: {:?}", key, self.name)),
    }
  }

  pub fn contains(&self, key: &[u8]) -> Result<bool> {
    match self.handler.contains_key(key) {
      Ok(new_value) => Ok(new_value),
      Err(e) => Err(e).with_context(|| format!("Failed to read key status {:?} database: {:?}", key, self.name))
    }
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
      .ok_or(DatabaseError::CreateMetadata(name.to_string()))?;

    Ok(())
  }

  fn read_metadata(&self, name: &str) -> Result<()> {
    let mut metadata = self
      .handler
      .get(b"metadata")?
      .ok_or(DatabaseError::ReadMetadata(name.to_string()))?;

    let metadata = simd_json::to_owned_value(&mut metadata)?;
    let db_field = match metadata.get("database") {
      None => Err(DatabaseError::InvalidMetadata(
        name.to_string(),
        String::from("database"),
      )),
      Some(value) => Ok(value),
    }?;

    if let Some(value) = db_field.as_object() {
      // TODO: всякие там проверки
    } else {
      return Err(DatabaseError::InvalidMetadata(
        name.to_string(),
        String::from("database"),
      ))?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::tempdir;

  #[test]
  fn tdd_new_database_with_metadata() {
    let dir = tempdir().unwrap();
    let db = Database::new(dir.path(), "settings").unwrap();
  }
}
