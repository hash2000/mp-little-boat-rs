use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalingEvent {
  ClientConnected { client_id: String },
  ClientDisconnected { client_id: String },
  MessageForwarded { from: String, to: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalingMessage {
  Offer { sdp: String },
  Answer { sdp: String },
  IceCandidate { candidate: String },
  Join { user_id: String },
  Leave { user_id: String },
  Ping,
  Pong,
}