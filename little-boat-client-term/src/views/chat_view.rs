use crate::views::chat_contacts_view::ChatContactsView;
use crate::views::frame::{DrawnView, FocusedView};
use std::rc::Rc;

use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout, Rect},
};

pub struct ChatView {
  contacts: ChatContactsView,
}

impl ChatView {
  pub fn new() -> Self {
    Self {
      contacts: ChatContactsView::new(),
    }
  }
}

impl DrawnView for ChatView {
  fn draw(&self, f: &mut Frame, _: Rect) {
    // rects of the chat view area
    // - left rect - contacts
    // - right rect - devided into two
    // -- bottom rect - messages from current contact
    // -- top rect - only buttons line for the current message
    let chunks = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
      .split(f.area());

    self.contacts.draw(f, chunks[0]);
  }
}
