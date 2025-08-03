use crate::test::test_config::TestConfig;

use futures::{SinkExt, StreamExt};
use little_boat_abstractions::{ControlEvent, IConfigReader, IService};
use little_boat_service_signaling::SignalingService;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn test_message_broadcasting() {
  // Config
  let temp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
  let addr = temp_listener.local_addr().unwrap();
  drop(temp_listener);
  #[derive(Clone)]
  struct TestConfigForBroadcast(u16);
  impl IConfigReader for TestConfigForBroadcast {
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
  let config_for_broadcast = Box::new(TestConfigForBroadcast(addr.port()));

  let service = SignalingService;
  let (service_tx, _service_rx) = broadcast::channel(100);
  let control_tx = broadcast::channel(100).0;
  let control_rx = control_tx.subscribe();

  let handle = service.start(service_tx.clone(), control_rx, config_for_broadcast).await.unwrap();

  // run wait time
  tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

  // two clients connections
  let client1_res = tokio_tungstenite::connect_async(format!("ws://{}", addr)).await;
  assert!(client1_res.is_ok());
  let (mut client1_ws_stream, _) = client1_res.unwrap();

  let client2_res = tokio_tungstenite::connect_async(format!("ws://{}", addr)).await;
  assert!(client2_res.is_ok());
  let (mut client2_ws_stream, _) = client2_res.unwrap();

  // wait wor clients register
  tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

  // client 1: send text
  let test_text = "Broadcast message from client 1";
  let send_text_res = client1_ws_stream.send(Message::Text(test_text.into())).await;
  assert!(send_text_res.is_ok());

  // client 2: recieve message
  let mut text_received_by_client2 = false;
  loop {
    tokio::select! {
        msg_res = client2_ws_stream.next() => {
            match msg_res {
                Some(Ok(Message::Text(text))) => {
                    if text == test_text {
                        text_received_by_client2 = true;
                        break;
                    }
                }
                Some(Ok(_)) => {
                    // Ignore
                }
                Some(Err(e)) => {
                    panic!("Client 2 error receiving message: {}", e);
                }
                None => {
                    panic!("Client 2 connection closed unexpectedly");
                }
            }
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)) => {
            break; // Таймаут
        }
    }
  }
  assert!(text_received_by_client2, "Text message was not broadcast to client 2");

  // client 1: send binary
  let test_binary = vec![0x01, 0x02, 0x03, 0xFF];
  let send_binary_res = client1_ws_stream.send(Message::Binary(test_binary.clone().into())).await;
  assert!(send_binary_res.is_ok());

  // client 2: reecieve binary
  let mut binary_received_by_client2 = false;
  loop {
    tokio::select! {
        msg_res = client2_ws_stream.next() => {
            match msg_res {
                Some(Ok(Message::Binary(data))) => {
                    if data.as_ref() == &test_binary[..] {
                        binary_received_by_client2 = true;
                        break;
                    }
                }
                Some(Ok(_)) => {
                    // Ignore
                }
                Some(Err(e)) => {
                    panic!("Client 2 error receiving binary message: {}", e);
                }
                None => {
                    panic!("Client 2 connection closed unexpectedly");
                }
            }
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)) => {
            break;
        }
    }
  }
  assert!(binary_received_by_client2, "Binary message was not broadcast to client 2");

  // clients disconnected
  let _ = client1_ws_stream.close(None).await;
  let _ = client2_ws_stream.close(None).await;

  // stop service
  let _ = control_tx.send(ControlEvent::Stop("signaling".to_string()));
  let service_result = tokio::time::timeout(Duration::from_secs(5), handle).await;
  assert!(service_result.is_ok());
  let service_handle_result = service_result.unwrap();
  assert!(service_handle_result.is_ok());
}
