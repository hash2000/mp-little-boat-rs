use little_boat_services::ServiceEvent;

use tokio::sync::mpsc;

pub fn run_services(tx: mpsc::UnboundedSender<ServiceEvent>) {
  //  run_webrtc_service(tx);
}
