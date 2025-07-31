use crate::views::{EventsHandledView, ViewContext};
use crate::views::chat::chat_contacts_view::ChatContactsView;
use crate::views::chat::chat_messages_view::ChatMessagesView;
use crate::views::frame::{DrawnView, FocusedView};

use crossterm::event::Event;
use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout, Rect},
};

pub struct ChatBoardView {
  contacts: ChatContactsView,
  messages: ChatMessagesView,
}

impl ChatBoardView {
  pub fn new() -> Self {
    Self {
      contacts: ChatContactsView::new(),
      messages: ChatMessagesView::new(),
    }
  }

  pub fn swap_focus(&mut self) {
    if self.contacts.has_focus() {
      self.contacts.set_focus(false);
      self.messages.set_focus(true);
    } else {
      self.contacts.set_focus(true);
      self.messages.set_focus(false);
    }
  }

  pub fn pool_event(&mut self, event: &Event) -> bool {
    self.contacts.handle_event(event) ||
    self.messages.handle_event(event)
  }
}

impl FocusedView for ChatBoardView {
  fn set_focus(&mut self, set: bool) {
    self.contacts.set_focus(set);
    self.messages.set_focus(set);
  }

  fn has_focus(&self) -> bool {
    self.contacts.has_focus() || self.messages.has_focus()
  }
}

impl DrawnView for ChatBoardView {
  fn draw(&self, f: &mut Frame, _: Rect, context: &mut dyn ViewContext) {
    // rects of the chat view area
    // - left rect - contacts
    // - right rect - devided into two
    // -- bottom rect - messages from current contact
    // -- top rect - only buttons line for the current message
    let chunks = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
      .split(f.area());

    self.contacts.draw(f, chunks[0], context);
    self.messages.draw(f, chunks[1], context);
  }
}
