use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatEvent {
  MessageReceived { from: String, content: String },
  UserJoined { user: String },
  UserLeft { user: String },
}