use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::IConfigReader;

#[derive(Debug, Clone)]
pub enum ControlEvent {
  Start(String),
  Stop(String),
  Shutdown,
}

#[derive(Debug, Clone)]
pub struct ServiceEventMessage {
  pub service: String,
  pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatEvent {
    MessageReceived { from: String, content: String },
    UserJoined { user: String },
    UserLeft { user: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    ServiceStarted { name: String },
    ServiceStopped { name: String },
    Error { service: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalingEvent {
    ClientConnected { client_id: String },
    ClientDisconnected { client_id: String },
    MessageForwarded { from: String, to: String },
}

#[derive(Debug, Clone)]
pub enum ServiceEvent {
  Signaling(SignalingEvent),
  Chat(ChatEvent),
  System(SystemEvent),
  Status(ServiceEventMessage),
  Error(ServiceEventMessage)
}

#[async_trait]
pub trait IService: Send + Sync {
    fn name(&self) -> &'static str;
    async fn start(
        &self,
        service_tx: broadcast::Sender<ServiceEvent>,
        control_rx: broadcast::Receiver<ControlEvent>,
        config: Box<dyn IConfigReader>,
    ) -> anyhow::Result<tokio::task::JoinHandle<anyhow::Result<()>>>;
}