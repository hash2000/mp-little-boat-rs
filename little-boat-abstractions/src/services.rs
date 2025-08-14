mod chat;
mod control;
mod signaling;
mod system;

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::broadcast;

use crate::IConfigReader;

pub use crate::services::chat::{
  ChatEvent, ChatMessage, ChatPeerConnections, ChatReceiver, ChatSender,
};
pub use crate::services::control::ControlEvent;
pub use crate::services::signaling::{SignalingEvent, SignalingMessage, SignalingPeers};
pub use crate::services::system::SystemEvent;

#[derive(Debug, Clone)]
pub struct ServiceEventMessage {
  pub service: String,
  pub message: String,
}

#[derive(Debug, Clone)]
pub enum ServiceEvent {
  Signaling(SignalingEvent),
  Chat(ChatEvent),
  System(SystemEvent),
  Status(ServiceEventMessage),
  Error(ServiceEventMessage),
}

#[async_trait]
pub trait IService: Send + Sync {
  fn name(&self) -> &'static str;
  async fn start(
    &self,
    service_tx: broadcast::Sender<ServiceEvent>,
    control_rx: broadcast::Receiver<ControlEvent>,
    config: Arc<dyn IConfigReader>,
  ) -> anyhow::Result<tokio::task::JoinHandle<anyhow::Result<()>>>;
}
