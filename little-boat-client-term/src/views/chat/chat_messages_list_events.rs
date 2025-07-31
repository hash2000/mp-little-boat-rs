use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::services::ServiceEvent;
use crate::views::{ChatBoardView, EventsHandledView, FocusedView};
use crate::views::{chat::chat_messages_list_view::ChatMessagesListView};

impl EventsHandledView for ChatMessagesListView {

  fn handle_service_event(&mut self, event: &ServiceEvent) {
    
  }

  fn handle_event(&mut self, event: &Event) -> bool {
    if !self.has_focus() {
      return false;
    }

    if let Event::Key(key) = event {
      if key.kind == KeyEventKind::Press && key.code == KeyCode::Up {
        
      }

      if key.kind == KeyEventKind::Press && key.code == KeyCode::Down {
        
      }
    }

    false
  }

}