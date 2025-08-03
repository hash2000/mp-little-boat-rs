use little_boat_abstractions::{ControlEvent, ServiceEvent};
use tokio::sync::{broadcast, mpsc};

use crate::config::Config;

pub fn start_signaling_service(
  tx: mpsc::UnboundedSender<ServiceEvent>,
  mut shutdown: broadcast::Receiver<ControlEvent>,
  cfg: &Config,
) -> tokio::task::JoinHandle<()> {

  

  tokio::spawn(async move {


  })
}
