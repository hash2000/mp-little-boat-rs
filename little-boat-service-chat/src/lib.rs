mod chat;
mod chat_messages;

use async_trait::async_trait;
use little_boat_abstractions::{
  ControlEvent, IConfigReader, IService, ServiceEvent,
};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::broadcast;

pub use crate::chat::ChatService;

#[async_trait]
impl IService for ChatService {
  fn name(&self) -> &'static str {
    "chat"
  }

  async fn start(
    &self,
    service_tx: broadcast::Sender<ServiceEvent>,
    control_rx: broadcast::Receiver<ControlEvent>,
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

    let self_arc = Arc::new(self.clone_inner());

    let handle = tokio::spawn(async move {
        self_arc.run_chat_service(
          service_tx,
          control_rx,
          signaling_url,
          stun_server,
          user_id,
          service_name,
      ).await
    });

    Ok(handle)
  }
}