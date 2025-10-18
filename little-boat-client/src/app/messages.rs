use std::time::Instant;
use iced::window;

#[derive(Debug, Clone)]
pub enum Message {
  WindowOpened(window::Id),
  Window(window::Id, window::Event),
  Tick(Instant),
}
