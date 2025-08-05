use little_boat_abstractions::SignalingMessage;



#[tokio::test]
async fn test_ping_message_handling() {
  let msg = SignalingMessage::Ping;

  let json_result = simd_json::to_string(&msg);
  assert!(json_result.is_ok());

  let json = json_result.unwrap();
  let mut bytes = json.as_bytes().to_vec();
  let parsed_result = simd_json::from_slice::<SignalingMessage>(&mut bytes);
  assert!(parsed_result.is_ok());

  if let SignalingMessage::Ping = parsed_result.unwrap() {
    // Success
  } else {
    panic!("Ожидался Ping");
  }
}
