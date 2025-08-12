use crossterm::event::KeyEvent;
use ratatui::prelude::*;

pub trait WidgetRenderer {
  fn render(&self, frame: &mut Frame, area: Rect);

  fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<bool>;
  
  fn get_name(&self) -> &'static str;
}
