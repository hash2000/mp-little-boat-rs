use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
  ServiceStarted { name: String },
  ServiceStopped { name: String },
  Error { service: String, message: String },
}

