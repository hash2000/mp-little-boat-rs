use async_trait::async_trait;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use little_boat_abstractions::{ControlEvent, IConfigReader, IService, ServiceEvent};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio_tungstenite::{
  accept_async,
  tungstenite::protocol::Message,
};

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SignalingMessage {
  Offer { sdp: String },
  Answer { sdp: String },
  IceCandidate { candidate: String },
  Ping,
  Pong,
}

pub struct SignalingService;

#[async_trait]
impl IService for SignalingService {
  fn name(&self) -> &'static str {
    "signaling"
  }

  async fn start(
    &self,
    service_tx: broadcast::Sender<ServiceEvent>,
    mut control_rx: broadcast::Receiver<ControlEvent>,
    config: Box<dyn IConfigReader>,
  ) -> anyhow::Result<tokio::task::JoinHandle<anyhow::Result<()>>> {

    let service_name = self.name().to_string();
    let host = config.get_str(b"service.signaling.host", "127.0.0.1");
    let port: u64 = config.get_int(b"service.signaling.port", 8080) as u64;
    let addr = format!("{}:{}", host, port);
    let service_name_clone = service_name.clone();
    let service_tx_clone = service_tx.clone();

    let handle = tokio::spawn(async move {      
      little_boat_abstractions::log_info!(service_name_clone, "Starting signaling service");

      let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", addr, e))?;

      little_boat_abstractions::log_info!(service_name_clone, "Listening on {}", addr);

      // starting event
      let _ = service_tx_clone.send(ServiceEvent::System(
        little_boat_abstractions::SystemEvent::ServiceStarted { name: service_name_clone.clone() },
      ));

      let peers: Peers = Arc::new(Mutex::new(HashMap::new()));

      loop {
        tokio::select! {
            // processiong connections
            accept_result = listener.accept() => {
                match accept_result {
                    Ok((stream, _)) => {
                        let peers = Arc::clone(&peers);
                        let service_tx = service_tx_clone.clone();
                        let service_name = service_name_clone.clone();

                        tokio::spawn(async move {
                            if let Err(e) = handle_client_connection(stream, peers, service_tx, service_name).await {
                                little_boat_abstractions::log_error!("signaling", "Client connection error: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        little_boat_abstractions::log_error!(service_name_clone, "Accept error: {}", e);
                    }
                }
            }

            // processing control events
            control_result = control_rx.recv() => {
                match control_result {
                    Ok(ControlEvent::Stop(name)) if name == service_name => {
                        little_boat_abstractions::log_info!(service_name_clone, "Received stop command");
                        break;
                    }
                    Ok(ControlEvent::Shutdown) => {
                        little_boat_abstractions::log_info!(service_name_clone, "Received shutdown command");
                        break;
                    }
                    Ok(_) => {
                      // Ignore
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        little_boat_abstractions::log_info!(service_name_clone, "Control channel closed");
                        break;
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        little_boat_abstractions::log_warn!(service_name_clone, "Control channel lagged");
                    }
                }
            }
        }
      }

      little_boat_abstractions::log_info!(service_name_clone, "Signaling service stopped");
      Ok(())
    });

    Ok(handle)
  }
}

async fn handle_client_connection(
  stream: tokio::net::TcpStream,
  peers: Peers,
  service_tx: broadcast::Sender<ServiceEvent>,
  service_name: String,
) -> anyhow::Result<()> {
  let ws_stream = accept_async(stream).await?;
  let (ws_sender, mut ws_receiver) = ws_stream.split();

  let client_id = format!("client_{}", uuid::Uuid::new_v4());

  {
    let mut p = peers.lock().await;
    p.insert(client_id.clone(), ws_sender);
  }

  little_boat_abstractions::log_info!(&service_name, "Client connected: {}", client_id);

  let _ = service_tx.send(ServiceEvent::Signaling(
    little_boat_abstractions::SignalingEvent::ClientConnected { client_id: client_id.clone() },
  ));

  while let Some(msg) = ws_receiver.next().await {
    match msg {
      Ok(Message::Text(text)) => {
        let mut bytes = text.as_bytes().to_vec();
        match simd_json::from_slice::<SignalingMessage>(&mut bytes) {
          Ok(sig_msg) => {
            match sig_msg {
              SignalingMessage::Ping => {
                let pong = SignalingMessage::Pong;
                if let Ok(json) = simd_json::to_string(&pong) {
                  let _ = send_to_client(&client_id, &peers, &json).await;
                }
              }
              _ => {
                // send rest signaling messages to all clients
                let _ = broadcast_message(&peers, &client_id, &text).await?;
              }
            }
          }
          Err(_) => {
            // send common messages to all clients
            let _ = broadcast_message(&peers, &client_id, &text).await?;
          }
        }
      }
      Ok(Message::Binary(data)) => {
        // send binary data to all clients
        let _ = broadcast_binary(&peers, &client_id, data.to_vec()).await?;
      }
      Ok(Message::Close(_)) => {
        break;
      }
      Ok(Message::Ping(data)) => {
        let _ = send_pong_to_client(&client_id, &peers, data.to_vec()).await?;
      }
      Ok(Message::Pong(_)) => {
        // Ignore pong
      }
      Ok(Message::Frame(_)) => {
        // Шптщку Frame, since it should not appear at this API level.
        // https://docs.rs/tungstenite/latest/tungstenite/enum.Message.html#variant.Frame
        little_boat_abstractions::log_warn!(
          &service_name,
          "Unexpected Message::Frame received and ignored."
        );
      }
      Err(e) => {
        little_boat_abstractions::log_error!(&service_name, "WebSocket error: {}", e);
        return Err(e.into());
      }
    }
  }

  // remove client
  {
    let mut p = peers.lock().await;
    p.remove(&client_id);
  }

  little_boat_abstractions::log_info!(&service_name, "Client disconnected: {}", client_id);

  // send disconnect event
  let _ = service_tx.send(ServiceEvent::Signaling(
    little_boat_abstractions::SignalingEvent::ClientDisconnected { client_id: client_id.clone() },
  ));

  Ok(())
}

async fn broadcast_message(peers: &Peers, from_client: &str, message: &str) -> anyhow::Result<()> {
  let mut p = peers.lock().await;
  let mut to_remove = Vec::new();

  for (id, sender) in p.iter_mut() {
    if *id != from_client {
      if let Err(_) = sender.send(Message::Text(message.into())).await {
        to_remove.push(id.clone());
        todo!("log this error");
        //little_boat_abstractions::log_warn!(&service_name_from_context, "Failed to send message to client {}: {}", id, err);
      }
    }
  }

  // remove disconnected clients
  for id in to_remove {
    p.remove(&id);
  }

  Ok(())
}

async fn broadcast_binary(peers: &Peers, from_client: &str, data: Vec<u8>) -> anyhow::Result<()> {
  let data_bytes: Bytes = data.into(); // cast Vec to Bytes once
  let mut p = peers.lock().await;
  let mut to_remove = Vec::new();
  for (id, sender) in p.iter_mut() {
    if *id != from_client {
      if let Err(_) = sender.send(Message::Binary(data_bytes.clone())).await {
        to_remove.push(id.clone());
        todo!("log this error");
      }
    }
  }

  // remove disconnected clients
  for id in to_remove {
    p.remove(&id);
  }
  Ok(())
}

async fn send_to_client(client_id: &str, peers: &Peers, message: &str) -> anyhow::Result<()> {
  let mut p = peers.lock().await;
  if let Some(sender) = p.get_mut(client_id) {
    let _ = sender.send(Message::Text(message.into())).await;
  }
  Ok(())
}

async fn send_pong_to_client(client_id: &str, peers: &Peers, data: Vec<u8>) -> anyhow::Result<()> {
  let mut p = peers.lock().await;
  if let Some(sender) = p.get_mut(client_id) {
    let _ = sender.send(Message::Pong(data.into())).await;
  }
  Ok(())
}
