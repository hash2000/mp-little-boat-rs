use crate::database::Database;
use anyhow::Result;
use std::path::Path;

pub struct OptionsDb {
  db: Database,
}

impl OptionsDb {
  pub fn new(path: &Path, name: &str) -> Result<Self> {
    let db = OptionsDb {
      db: Database::new(path, name, None)?,
    };

    Ok(db)
  }

  pub fn get(&self, key: &[u8]) -> Result<Option<simd_json::OwnedValue>> {
    self.db.get_json(key)
  }

  pub fn set(self, key: &[u8], value: simd_json::OwnedValue) -> Result<()> {
    self.db.set_json(key, value)
  }
}
