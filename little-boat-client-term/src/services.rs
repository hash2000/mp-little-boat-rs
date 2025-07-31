mod webrtc_service;

use tokio::sync::mpsc;

use crate::services::webrtc_service::run_webrtc_service;

pub struct ServiceEvent {
  
}

pub fn run_services(tx: mpsc::UnboundedSender<ServiceEvent>) {
  run_webrtc_service(tx);
}