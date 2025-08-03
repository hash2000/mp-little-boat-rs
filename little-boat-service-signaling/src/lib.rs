use anyhow::Context;
use async_trait::async_trait;
use little_boat_abstractions::{ControlEvent, IConfigReader, IService, ServiceEvent};
use tokio::net::ToSocketAddrs;
use tokio::sync::{broadcast, mpsc};

use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::f32::consts::E;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use webrtc::dtls::listener;

type Peers = Arc<
  Mutex<
    HashMap<
      String,
      futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        Message,
      >,
    >,
  >,
>;

pub struct SignalingService;

#[async_trait]
impl IService for SignalingService {
  fn name(&self) -> &'static str {
    "signaling"
  }

  async fn start(
    &self,
    service_tx: broadcast::Sender<ServiceEvent>,
    control_rx: broadcast::Receiver<ControlEvent>,
    config: &dyn IConfigReader,
  ) -> anyhow::Result<tokio::task::JoinHandle<anyhow::Result<()>>> {
    let service_name = self.name().to_string();
    let handle = tokio::spawn(async move {
      little_boat_abstractions::log_info!(service_name, "Starting signaling service");

      // let host = config.get_str(b"service.signaling.host", "127.0.0.1");
      // let port: u64 = config.get_int(b"service.signaling.port", 8080) as u64;
      // let addr = format!("{}:{}", host, port);

      // let listener = tokio::net::TcpListener::bind(&addr)
      //   .await
      //   .context(format!("Failed to bind to {}", addr))?;

      Ok(())
    });

    Ok(handle)
  }
}
