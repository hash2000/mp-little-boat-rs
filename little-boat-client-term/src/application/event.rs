use little_boat_abstractions::ServiceEvent;

use anyhow::Result;
use crossterm::event::Event;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum AppEvent {
  Key(crossterm::event::KeyEvent),
  Service(ServiceEvent),
  Tick,
}

pub async fn poll_event(tx: mpsc::UnboundedSender<AppEvent>) -> Result<()> {
  if crossterm::event::poll(std::time::Duration::from_millis(100))? {
    if let Event::Key(key) = crossterm::event::read()? {
      tx.send(AppEvent::Key(key))?;
    }
  }
  Ok(())
}
