use crossterm::event::Event;

use crate::views::{chat_contacts_view::ChatContactsView, EventsHandledView};

impl EventsHandledView for ChatContactsView {

  fn handle_event(&mut self, event: &Event) -> bool {
    true
  }

}