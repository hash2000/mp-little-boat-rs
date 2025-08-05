use crate::test::test_config::TestConfig;

use std::{sync::Arc, time::Duration};
use little_boat_abstractions::{ControlEvent, IService};
use little_boat_service_signaling::SignalingService;
use tokio::sync::broadcast;

  #[tokio::test]
  async fn test_signaling_service_start_stop() {
    let service = SignalingService;
    let (service_tx, _service_rx) = broadcast::channel(100);
    let control_tx = broadcast::channel(100).0;
    let control_rx = control_tx.subscribe();
    let config = Arc::new(TestConfig);

    // Start service
    let result = service.start(service_tx, control_rx, config).await;
    assert!(result.is_ok());

    let handle = result.unwrap();

    // Send command
    let _ = control_tx.send(ControlEvent::Stop("signaling".to_string()));

    // wait the handler
    let result = tokio::time::timeout(Duration::from_secs(5), handle).await;

    assert!(result.is_ok());
  }