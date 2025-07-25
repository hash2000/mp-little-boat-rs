use crate::database::{self, Database};
use simd_json::base::ValueAsObject;
use simd_json::derived::ValueObjectAccess;
use simd_json::{json, to_vec};

const METADATA_VERSION: &'static str = "0.0.1";

pub fn initialize(db: &Database, name: &str) -> anyhow::Result<()> {
  let read_result = read(db, name);

  // metadata isn't read. i'm trying to write new once
  if read_result.is_err() {
    append(db, name)?;
  }

  Ok(())
}

fn read(db: &Database, name: &str) -> anyhow::Result<()> {
  let mut metadata = db.handler.get(b"metadata")?.ok_or(anyhow::anyhow!(
    "Can't read metadata from database '{:?}'",
    name
  ))?;

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

  Ok(())
}

fn append(db: &Database, name: &str) -> anyhow::Result<()> {
  let metadata = json!({
    "database": {
      "name": name,
      "version": METADATA_VERSION
    }
  });

  let metadata = to_vec(&metadata)?;
  db.handler
    .insert(b"metadata", metadata)?
    .ok_or(anyhow::anyhow!(
      "Can't create metadata for database [{:?}]",
      name
    ))?;

  Ok(())
}
