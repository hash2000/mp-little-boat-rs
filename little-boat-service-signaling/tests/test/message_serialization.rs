use little_boat_service_signaling::SignalingMessage;

#[tokio::test]
async fn test_signaling_message_serialization() {
  let msg = SignalingMessage::Offer { sdp: "test_sdp".to_string() };

  // Тестируем сериализацию с simd-json
  let json_result = simd_json::to_string(&msg);
  assert!(json_result.is_ok());

  let json = json_result.unwrap();
  let mut bytes = json.as_bytes().to_vec();
  let parsed_result = simd_json::from_slice::<SignalingMessage>(&mut bytes);
  assert!(parsed_result.is_ok());

  if let SignalingMessage::Offer { sdp } = parsed_result.unwrap() {
    assert_eq!(sdp, "test_sdp");
  } else {
    panic!("Ожидался Offer");
  }
}
