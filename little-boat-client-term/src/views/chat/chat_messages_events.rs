use crate::{services::ServiceEvent, views::{
  chat::chat_messages_view::ChatMessagesView, 
  EventsHandledView,
  FocusedView
}};

use crossterm::event::Event;

impl EventsHandledView for ChatMessagesView {
  
  fn handle_service_event(&mut self, event: &ServiceEvent) {
    
  }

  fn handle_event(&mut self, event: &Event) -> bool {
    if !self.has_focus() {
      return false;
    }

    self.pool_event(event)
  }

}