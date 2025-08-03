use crate::views::{ChatBoardView, EventsHandledView, FocusedView};
use little_boat_abstractions::ServiceEvent;

use crossterm::event::{Event, KeyCode, KeyEventKind};

impl EventsHandledView for ChatBoardView {
  fn handle_service_event(&mut self, event: &ServiceEvent) {}

  fn handle_event(&mut self, event: &Event) -> bool {
    if !self.has_focus() || self.pool_event(event) {
      return false;
    }

    if let Event::Key(key) = event {
      if key.kind == KeyEventKind::Press && key.code == KeyCode::Tab {
        self.swap_focus();
        return true;
      }
    }

    false
  }
}
