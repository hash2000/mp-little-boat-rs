use async_trait::async_trait;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use little_boat_abstractions::ChatPeerConnections;
use little_boat_abstractions::{
  ControlEvent, IConfigReader, IService, ServiceEvent, SignalingEvent, SignalingMessage,
  SignalingPeers, SystemEvent,
};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};


pub struct ChatService {
    peer_connections: ChatPeerConnections,
}

impl ChatService {
    pub fn new() -> Self {
        Self {
            peer_connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl IService for ChatService {
  fn name(&self) -> &'static str {
    "chat"
  }

  async fn start(
    &self,
    service_tx: broadcast::Sender<ServiceEvent>,
    mut control_rx: broadcast::Receiver<ControlEvent>,
    config: Arc<dyn IConfigReader>,
  ) -> anyhow::Result<tokio::task::JoinHandle<anyhow::Result<()>>> {
    let service_name = self.name().to_string();

    // read configs
    let signaling_url = config.get_str(b"service.chat.signaling.url", "ws://127.0.0.1:8080");
    let stun_server = config.get_str(b"service.chat.stun.url", "stun:stun.l.google.com:19302");

    // generate user identity
    let user_id = format!("user_{}", Uuid::new_v4());

    // clone shared valuses
    let service_name = service_name.clone();
    let service_tx = service_tx.clone();
    let peer_connections = self.peer_connections.clone();

    let handle = tokio::spawn(async move {
      little_boat_abstractions::log_info!(service_name, 
        "Starting chat service for user: {}", user_id);
      

      Ok(())
    });

    Ok(handle)
  }
}