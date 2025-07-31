use crossterm::event::Event;

use crate::views::{chat_messages_list_view::ChatMessagesListView, EventsHandledView};

impl EventsHandledView for ChatMessagesListView {

  fn handle_event(&mut self, event: &Event) -> bool {
    true
  }

}