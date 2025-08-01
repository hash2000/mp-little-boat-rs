use crate::{
  views::{
    chat::chat_messasge_panel_view::ChatMessagePanelView, 
    EventsHandledView,
    FocusedView,
  }
};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use little_boat_services::ServiceEvent;

impl EventsHandledView for ChatMessagePanelView {

  fn handle_service_event(&mut self, event: &ServiceEvent) {
    
  }

  fn handle_event(&mut self, event: &Event) -> bool {
    if !self.has_focus() {
      return false;
    }

    if let Event::Key(key) = event {
      if key.kind == KeyEventKind::Press && key.code == KeyCode::Right {
        self.select_next_button();
      }

      if key.kind == KeyEventKind::Press && key.code == KeyCode::Left {
        self.select_prev_button();
      }

      if key.kind == KeyEventKind::Press && (key.code == KeyCode::Enter || key.code == KeyCode::Char(' ')) {
        self.process_current_button();
      }
    }

    false
  }
}
