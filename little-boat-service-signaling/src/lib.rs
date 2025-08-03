use little_boat_abstractions::ServiceEvent;
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc;

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

pub struct SignalingServiceConfig {
  addr: String,
}

impl SignalingServiceConfig {
  pub fn new() -> Self {
    Self { addr: "127.0.0.1:8080".to_string() }
  }

  pub fn addr(&self) -> String {
    self.addr.clone()
  }
}

pub fn run_service(tx: mpsc::UnboundedSender<ServiceEvent>, config: SignalingServiceConfig) {
  tokio::spawn(async move {
    // let listener = match tokio::net::TcpListener::bind(config.addr()).await {
    //   Err(e) => {
    //     //let _ = tx.send(ServiceEvent::Error(e));
    //     return;
    //   }
    //   Ok(value) => value,
    // };

    // let peers: Peers = Arc::new(Mutex::new(HashMap::new()));

    // while let Ok((stream, _)) = listener.accept().await {
    //   let peers = Arc::clone(&peers);
    // }
  });
}
