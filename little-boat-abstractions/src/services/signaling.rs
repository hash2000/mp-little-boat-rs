use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

use serde::{Deserialize, Serialize};

pub type SignalingPeers = Arc<
  Mutex<
    HashMap<
      // Client ID
      String,
      // WebSocket stream sender
      futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        Message,
      >,
    >,
  >,
>;

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