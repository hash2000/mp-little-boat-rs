use async_trait::async_trait;
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

#[derive(Debug, Clone)]
pub enum ServiceEvent {
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
        config: &dyn IConfigReader,
    ) -> anyhow::Result<tokio::task::JoinHandle<anyhow::Result<()>>>;
}