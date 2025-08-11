use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ServiceStatus {
  pub name: String,
  pub status: String,
  pub last_update: Instant,
}
