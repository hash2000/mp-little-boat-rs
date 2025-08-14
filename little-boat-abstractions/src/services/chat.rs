use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

pub type ChatPeerConnections = Arc<
  Mutex<
    HashMap<
      // user identity
      String,
      // connections
      webrtc::peer_connection::RTCPeerConnection
    >
  >
>;

pub type ChatSender = futures::stream::SplitSink<
  tokio_tungstenite::WebSocketStream<
    tokio::net::TcpStream
  >,
  Message
>;

pub type ChatReceiver = futures::stream::SplitStream<
  tokio_tungstenite::WebSocketStream<
    tokio::net::TcpStream
  >
>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatEvent {
  MessageReceived { from: String, content: String },
  UserJoined { user: String },
  UserLeft { user: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatMessage {
    Text { from: String, content: String },
    System { content: String },
}