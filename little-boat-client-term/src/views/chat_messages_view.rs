use crate::views::{
  chat_messages_list_view::ChatMessagesListView, chat_messasge_panel_view::ChatMessagePanelView, frame::{DrawnView, EventsHandledView, FocusedView}, ViewContext
};
use crossterm::event::Event;
use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout, Rect},
};

pub struct ChatMessagesView {
  list: ChatMessagesListView,
  buttons: ChatMessagePanelView,
}

impl ChatMessagesView {
  pub fn new() -> Self {
    Self {
      list: ChatMessagesListView::new(),
      buttons: ChatMessagePanelView::new(),
    }
  }
}

impl FocusedView for ChatMessagesView {
  fn set_focus(&mut self, set: bool) {
    self.list.set_focus(set);
    self.buttons.set_focus(set);
  }

  fn has_focus(&self) -> bool {
    self.list.has_focus() || self.buttons.has_focus()
  }
}

impl DrawnView for ChatMessagesView {
  fn draw(&self, f: &mut Frame, area: Rect, context: &mut dyn ViewContext) {
    let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
      .split(area);

    self.list.draw(f, chunks[0], context);
    self.buttons.draw(f, chunks[1], context);
  }
}


impl EventsHandledView for ChatMessagesView {

  fn handle_event(&mut self, event: &Event) -> bool {
    true
  }

}