use async_trait::async_trait;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use little_boat_abstractions::ChatPeerConnections;
use little_boat_abstractions::{
  ControlEvent, IConfigReader, IService, ServiceEvent, SignalingEvent, SignalingMessage,
  SignalingPeers, SystemEvent,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use uuid::Uuid;

pub struct ChatService {
  pub(crate) peer_connections: ChatPeerConnections,
}

impl ChatService {
  pub fn new() -> Self {
    Self { peer_connections: Arc::new(Mutex::new(HashMap::new())) }
  }

  pub fn clone_inner(&self) -> Self {
    Self { peer_connections: Arc::clone(&self.peer_connections) }
  }

  pub async fn run_chat_service(
    self: Arc<Self>,
    service_tx: broadcast::Sender<ServiceEvent>,
    mut control_rx: broadcast::Receiver<ControlEvent>,
    signaling_url: String,
    stun_server: String,
    user_id: String,
    service_name: String,
  ) -> anyhow::Result<()> {
    Ok(())
  }
}
