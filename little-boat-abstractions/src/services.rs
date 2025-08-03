use tokio::sync::{broadcast, mpsc};
use async_trait::async_trait;

use crate::IConfigReader;

#[derive(Debug, Clone)]
pub enum ControlEvent {
  Start(String),
  Stop(String),
  Shutdown,
}

#[derive(Debug)]
pub struct ServiceEventMessage {
  pub service: String,
  pub message: String,
}

#[derive(Debug)]
pub enum ServiceEvent {
  Status(ServiceEventMessage),
  Error(ServiceEventMessage)
}

#[async_trait]
pub trait Service: Send + Sync {
    fn name(&self) -> &'static str;
    async fn start(
        &self,
        service_tx: mpsc::UnboundedSender<ServiceEvent>,
        control_rx: broadcast::Receiver<ControlEvent>,
        config: &dyn IConfigReader,
    ) -> tokio::task::JoinHandle<()>;
}