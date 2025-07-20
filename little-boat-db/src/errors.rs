use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
  #[error("Can't create metadata for database [{0}]")]
  CreateMetadata(String),

  #[error("Can't read metadata from database [{0}]")]
  ReadMetadata(String),

  #[error("Invalid metadata for database [{0}] field [{1}]")]
  InvalidMetadata(String, String),
}