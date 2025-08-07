use crate::database::Database;
use simd_json::base::ValueAsObject;
use simd_json::derived::ValueObjectAccess;
use simd_json::{json, to_vec};

pub const METADATA_VERSION: &'static str = "0.0.1";

// if a new database has been created, then return the true result
pub fn initialize(db: &mut Database, name: &str) -> anyhow::Result<()> {
  let read_result = read(db, name)?;

  // metadata isn't read. i'm trying to write new once
  if read_result {
    
    db.fresh = false;
  } else {
    append(db, name)?;

    // a new database has been initialized
    db.fresh = true;
  }

  Ok(())
}

fn read(db: &Database, name: &str) -> anyhow::Result<bool> {
  let metadata = db
    .handler
    .get(b"metadata")?;

  let mut metadata = match metadata {
    Some(value) => value,
    None => return Ok(false)
  };

  let metadata = simd_json::to_owned_value(&mut metadata)?;
  let Some(database_f) = metadata.get("database") else {
    return Err(anyhow::anyhow!(
      "Invalid metadata from database '{:?}', reason: 'database' field is undefined",
      name
    ));
  };

  let Some(database_f) = database_f.as_object() else {
    return Err(anyhow::anyhow!(
      "Invalid metadata from database '{:?}', reason: 'database' field is not a object",
      name
    ));
  };

  let Some(database_name_f) = database_f.get("name") else {
    return Err(anyhow::anyhow!(
      "Invalid metadata from database '{:?}', reason: 'database.name' field is undefined",
      name
    ));
  };

  if database_name_f != name {
    return Err(anyhow::anyhow!(
      "Invalid metadata from database '{:?}', reason: 'database.name' field value is invalid",
      name
    ));
  }

  let Some(database_version_f) = database_f.get("version") else {
    return Err(anyhow::anyhow!(
      "Invalid metadata from database '{:?}', reason: 'database.version' field is undefined",
      name
    ));
  };

  if database_version_f != METADATA_VERSION {
    return Err(anyhow::anyhow!(
      "Invalid metadata from database '{:?}', reason: 'database.version' field is invalid",
      name
    ));
  }

  Ok(true)
}

fn append(db: &Database, name: &str) -> anyhow::Result<bool> {
  let metadata = json!({
    "database": {
      "name": name,
      "version": METADATA_VERSION
    }
  });

  let metadata = to_vec(&metadata)?;
  let insert_result = db.handler
    .insert(b"metadata", metadata)?;

  match insert_result {
    Some(_) => Ok(false),
    None => Ok(true)
  }
}
