#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
  Initializing,
  Running,
  Shutdown,
}
