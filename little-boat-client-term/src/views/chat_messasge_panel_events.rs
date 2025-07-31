use crossterm::event::Event;

use crate::views::{chat_messasge_panel_view::ChatMessagePanelView, EventsHandledView};

impl EventsHandledView for ChatMessagePanelView {

  fn handle_event(&mut self, event: &Event) -> bool {
    true
  }

}