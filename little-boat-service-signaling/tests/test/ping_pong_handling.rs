use crate::test::test_config::TestConfig;

use futures::{SinkExt, StreamExt};
use little_boat_abstractions::{ControlEvent, IConfigReader, IService};
use little_boat_service_signaling::SignalingService;
use tokio_tungstenite::tungstenite::Message;
use std::{sync::Arc, time::Duration};
use tokio::sync::broadcast;

#[tokio::test]
async fn test_ping_pong_handling() {
  let temp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
  let addr = temp_listener.local_addr().unwrap();
  drop(temp_listener);
  
  #[derive(Clone)]
  struct TestConfigForPingPong(u16);
  impl IConfigReader for TestConfigForPingPong {
    fn has_flag(&self, key: &[u8], def: bool) -> bool {
      TestConfig::has_flag(&TestConfig, key, def)
    }
    fn get_str(&self, key: &[u8], def: &str) -> String {
      TestConfig::get_str(&TestConfig, key, def)
    }
    fn get_json(&self, key: &[u8]) -> Option<simd_json::OwnedValue> {
      TestConfig::get_json(&TestConfig, key)
    }
    fn get_bool(&self, key: &[u8], def: bool) -> bool {
      TestConfig::get_bool(&TestConfig, key, def)
    }
    fn get_float(&self, key: &[u8], def: f64) -> f64 {
      TestConfig::get_float(&TestConfig, key, def)
    }
    fn get_int(&self, key: &[u8], def: usize) -> usize {
      match key {
        b"service.signaling.port" => self.0 as usize,
        _ => TestConfig::get_int(&TestConfig, key, def),
      }
    }
  }

  let config_for_ping_pong = Arc::new(TestConfigForPingPong(addr.port()));

  let service = SignalingService;
  let (service_tx, _service_rx) = broadcast::channel(100);
  let control_tx = broadcast::channel(100).0;
  let control_rx = control_tx.subscribe();

  let handle = service.start(service_tx.clone(), control_rx, config_for_ping_pong).await.unwrap();

  // run wait timer
  tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

  // emulating a client connection
  let client_res = tokio_tungstenite::connect_async(format!("ws://{}", addr)).await;
  assert!(client_res.is_ok());
  let (mut client_ws_stream, _) = client_res.unwrap();

  // wait for client registration
  tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

  // send Ping
  let ping_data = bytes::Bytes::copy_from_slice(&[1, 2, 3, 4]);
  let send_ping_res = client_ws_stream.send(Message::Ping(ping_data.clone())).await;
  assert!(send_ping_res.is_ok(), "Failed to send Ping: {:?}", send_ping_res.err());

  // wait Pong
  let mut pong_received = false;
  let mut received_pong_data = Vec::new();
  loop {
    tokio::select! {
        msg_res = client_ws_stream.next() => {
            match msg_res {
                Some(Ok(Message::Pong(data))) => {
                    pong_received = true;
                    received_pong_data = data.into();
                    break;
                }
                Some(Ok(_)) => {
                    // Ignore
                }
                Some(Err(e)) => {
                    panic!("Error receiving message: {}", e);
                }
                None => {
                    panic!("Connection closed unexpectedly");
                }
            }
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)) => {
            break;
        }
    }
  }
  assert!(pong_received, "Pong message was not received");
  // The data in Pong may differ from the data in Ping depending on the server implementation,
  // but the standard practice is to return the same data.
  // However, in our code we just send Pong with the same data.
  assert_eq!(received_pong_data, ping_data, "Pong data mismatch");

  // client disconnection
  let _ = client_ws_stream.close(None).await;

  // stop service
  let _ = control_tx.send(ControlEvent::Stop("signaling".to_string()));
  let service_result = tokio::time::timeout(Duration::from_secs(5), handle).await;
  assert!(service_result.is_ok());
  let service_handle_result = service_result.unwrap();
  assert!(service_handle_result.is_ok());
}
