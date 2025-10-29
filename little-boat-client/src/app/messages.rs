use iced::window;
use std::time::Instant;

use crate::app::DashboardId;

#[derive(Debug, Clone)]
pub enum Message {
  WindowOpened(window::Id),
  Window(window::Id, window::Event),
  Tick(Instant),
  Event(window::Id, window::Event),

  ToggleSettings,
  ToggleMinimize,

  DashboardOpened { query: String },
  DashboardClosed(DashboardId),
  DashboardSelected(DashboardId),
}
