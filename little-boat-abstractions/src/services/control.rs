
#[derive(Debug, Clone)]
pub enum ControlEvent {
  Start(String),
  Stop(String),
  Shutdown,
}