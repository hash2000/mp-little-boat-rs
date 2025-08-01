use crossterm::event::{Event, KeyCode, KeyEventKind};
use little_boat_services::ServiceEvent;

use crate::views::{EventsHandledView, FocusedView};
use crate::views::{chat::chat_contacts_view::ChatContactsView};


impl EventsHandledView for ChatContactsView {
  
  fn handle_service_event(&mut self, event: &ServiceEvent) {
    
  }

  fn handle_event(&mut self, event: &Event) -> bool {
    if !self.has_focus() {
      return false;
    }

    if let Event::Key(key) = event {
      if key.kind == KeyEventKind::Press && key.code == KeyCode::Down {
        self.select_next();
      }

      if key.kind == KeyEventKind::Press && key.code == KeyCode::Up {
        self.select_prev();
      }

      if key.kind == KeyEventKind::Press && (key.code == KeyCode::Enter || key.code == KeyCode::Char(' ')) {
        self.choose_current();
      }
    }

    false
  }

}