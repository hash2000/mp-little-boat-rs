use crossterm::event::Event;

use crate::views::{chat_messages_view::ChatMessagesView, EventsHandledView};

impl EventsHandledView for ChatMessagesView {

  fn handle_event(&mut self, event: &Event) -> bool {
    true
  }

}