use crate::test::test_config::TestConfig;

use futures::SinkExt;
use little_boat_abstractions::{ControlEvent, IConfigReader, IService, ServiceEvent, SignalingEvent};
use little_boat_service_signaling::SignalingService;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn test_client_connect_disconnect() {
  
  use tokio_tungstenite::tungstenite::protocol::CloseFrame;

  let service = SignalingService;
  let (service_tx, _) = broadcast::channel(100);
  let control_tx = broadcast::channel(100).0;
  let control_rx = control_tx.subscribe();
  let config = Box::new(TestConfig);

  // start service
  let _ = service.start(service_tx.clone(), control_rx, config).await.unwrap();

  // run wait time
  tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

  // temp listener, for random port
  let temp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
  let addr = temp_listener.local_addr().unwrap();
  drop(temp_listener); 

  #[derive(Clone)]
  struct TestConfigWithPort(u16);
  impl IConfigReader for TestConfigWithPort {
    fn has_flag(&self, key: &[u8], def: bool) -> bool {
      match key {
        b"service.signaling.enabled" => true,
        _ => def,
      }
    }
    fn get_str(&self, key: &[u8], def: &str) -> String {
      match key {
        b"service.signaling.host" => "127.0.0.1".to_string(),
        _ => def.to_string(),
      }
    }
    fn get_json(&self, _key: &[u8]) -> Option<simd_json::OwnedValue> {
      None
    }
    fn get_bool(&self, _key: &[u8], def: bool) -> bool {
      def
    }
    fn get_float(&self, _key: &[u8], def: f64) -> f64 {
      def
    }
    fn get_int(&self, key: &[u8], def: usize) -> usize {
      match key {
        b"service.signaling.port" => self.0 as usize, 
        _ => def,
      }
    }
  }
  let config_with_port = Box::new(TestConfigWithPort(addr.port()));

  // restart service
  let (service_tx, mut service_rx) = broadcast::channel(100);
  let control_tx = broadcast::channel(100).0;
  let control_rx = control_tx.subscribe();
  let handle = service.start(service_tx.clone(), control_rx, config_with_port).await.unwrap();

  // Wait event ServiceStarted
  let mut service_started_received = false;
  while let Ok(event) = service_rx.recv().await {
    if let ServiceEvent::System(little_boat_abstractions::SystemEvent::ServiceStarted { name }) =
      event
    {
      if name == "signaling" {
        service_started_received = true;
        break;
      }
    }

    // timeout 
    tokio::select! {
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)) => {
            break;
        }
        _ = service_rx.recv() => {}
    }

  }
  assert!(service_started_received, "ServiceStarted event was not received");

  // client connection
  let client_res = tokio_tungstenite::connect_async(format!("ws://{}", addr)).await;
  assert!(client_res.is_ok(), "Failed to connect client: {:?}", client_res.err());
  let (mut client_ws_stream, _) = client_res.unwrap();

  // wait connection ClientConnected
  let mut client_connected_received = false;
  let client_id = loop {
    tokio::select! {
        event_res = service_rx.recv() => {
             if let Ok(ServiceEvent::Signaling(little_boat_abstractions::SignalingEvent::ClientConnected { client_id })) = event_res {
                 client_connected_received = true;
                 break client_id;
             }
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)) => {
            panic!("ClientConnected event was not received within timeout");
        }
    }
  };
  assert!(client_connected_received, "ClientConnected event was not received");

  // send client connection
  let test_message = "Hello from client!";
  let send_res = client_ws_stream.send(Message::Text(test_message.into())).await;
  assert!(send_res.is_ok(), "Failed to send message from client: {:?}", send_res.err());

  // client disconnect
  let close_res = client_ws_stream
    .close(Some(CloseFrame { code: 1000.into(), reason: "Test disconnect".into() }))
    .await;
  assert!(close_res.is_ok(), "Failed to close client connection: {:?}", close_res.err());

  // wait ewent ClientDisconnected
  let mut client_disconnected_received = false;
  loop {
    tokio::select! {
        event_res = service_rx.recv() => {
             if let Ok(ServiceEvent::Signaling(SignalingEvent::ClientDisconnected { client_id: disconnected_id })) = event_res {
                 assert_eq!(disconnected_id, client_id, "Disconnected client ID mismatch");
                 client_disconnected_received = true;
                 break;
             }
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)) => {
            panic!("ClientDisconnected event was not received within timeout");
        }
    }
  }
  assert!(client_disconnected_received, "ClientDisconnected event was not received");

  // send stop event
  let _ = control_tx.send(ControlEvent::Stop("signaling".to_string()));

  // stop service
  let service_result = tokio::time::timeout(Duration::from_secs(5), handle).await;
  assert!(service_result.is_ok(), "Service did not stop in time");
  let service_handle_result = service_result.unwrap();
  assert!(service_handle_result.is_ok(), "Service task failed: {:?}", service_handle_result.err());
}
