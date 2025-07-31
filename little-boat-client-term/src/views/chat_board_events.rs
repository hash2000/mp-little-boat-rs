use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::views::{ChatBoardView, EventsHandledView};

impl EventsHandledView for ChatBoardView {

  fn handle_event(&mut self, event: &Event) -> bool {
    if let Event::Key(key) = event { 
      if key.kind == KeyEventKind::Press {
        match key.code {
          KeyCode::Tab => self.swap_focus(),
          _ => ()
        }
      }
    }
    true
  }

}