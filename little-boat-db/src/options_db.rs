use crate::errors::DatabaseError;
use crate::database::Database;
use anyhow::Result;
use std::path::Path;

pub struct OptionsDb {
    db: Database,
}

impl OptionsDb {
    pub fn new(path: &Path, name: &str) -> Result<Self> {
        
        let db = OptionsDb {
            db: Database::new(path, name)?,
        };

        Ok(db)
    }
}