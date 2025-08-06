use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

pub type ChatPeerConnections = Arc<
  Mutex<
    HashMap<
      String,
      webrtc::peer_connection::RTCPeerConnection
    >
  >
>;

pub type ChatSocketSender = futures::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatEvent {
  MessageReceived { from: String, content: String },
  UserJoined { user: String },
  UserLeft { user: String },
}