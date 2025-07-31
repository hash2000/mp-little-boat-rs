use crate::views::{DrawnView, FocusedView, ViewContext};
use ratatui::{
  Frame,
  layout::Rect,
  widgets::{Block, Borders, Paragraph},
};

pub struct ChatMessagesListView {
  messages: Vec<String>,
  focused: bool,
}

impl ChatMessagesListView {
  pub fn new() -> Self {
    Self {
      messages: vec![
        "Привет!".to_string(),
        "Как дела?".to_string(),
        "Что нового?".to_string(),
      ],
      focused: false
    }
  }

}

impl FocusedView for ChatMessagesListView {
  fn set_focus(&mut self, set: bool) {
    self.focused = set;
  }

  fn has_focus(&self) -> bool {
    self.focused
  }
}

impl DrawnView for ChatMessagesListView {
  fn draw(&self, f: &mut Frame, area: Rect, ccontext: &mut dyn ViewContext) {
    let messages_block = Block::default().title("Messages").borders(Borders::ALL);
    let messages_text = self.messages.join("\n");
    let messages_paragraph = Paragraph::new(messages_text).block(messages_block);

    f.render_widget(messages_paragraph, area);
  }
}

