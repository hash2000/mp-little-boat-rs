use crate::views::chat_contacts_view::ChatContactsView;
use crate::views::chat_messages_view::ChatMessagesView;
use crate::views::frame::{DrawnView, FocusedView};
use crate::views::ViewContext;

use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout, Rect},
};

pub struct ChatView {
  contacts: ChatContactsView,
  messages: ChatMessagesView,
}

impl ChatView {
  pub fn new() -> Self {
    Self {
      contacts: ChatContactsView::new(),
      messages: ChatMessagesView::new(),
    }
  }
}

impl FocusedView for ChatView {
  fn set_focus(&mut self, set: bool) {
    self.contacts.set_focus(set);
    self.messages.set_focus(set);
  }

  fn has_focus(&self) -> bool {
    self.contacts.has_focus() || self.messages.has_focus()
  }
}

impl DrawnView for ChatView {
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
