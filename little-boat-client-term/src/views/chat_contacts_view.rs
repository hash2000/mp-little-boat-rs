use crate::views::{frame::{DrawnView, FocusedView}, ViewContext};
use ratatui::{
  Frame,
  layout::Rect,
  style::{Color, Style},
  text::{Line, Span},
  widgets::{Block, Borders, List, ListItem},
};

pub struct ChatContactsView {
  focused: bool,
  contacts: Vec<String>,
  selected: usize,
}

impl ChatContactsView {
  pub fn new() -> Self {
    Self {
      focused: false,
      contacts: vec![
        "Robot Werter".to_string(),
        "Incapsulacia".to_string(),
        "XXX Kaban".to_string(),
        "OgelMogel".to_string(),
        "Многоярусный Кардинал".to_string(),
      ],
      selected: 0,
    }
  }

  pub fn get_selected(&self) -> usize {
    self.selected
  }

  pub fn set_selected(&mut self, index: usize) {
    if index > 0 && index < self.contacts.len() - 1 {
      self.selected = index;
    }
  }
}

impl FocusedView for ChatContactsView {
  fn set_focus(&mut self, set: bool) {
    self.focused = set;
  }

  fn has_focus(&self) -> bool {
    self.focused
  }
}

impl DrawnView for ChatContactsView {
  fn draw(&self, f: &mut Frame, area: Rect, context: &mut dyn ViewContext) {
    let contacts_block = Block::default()
      .title("Contacts")
      .borders(Borders::ALL)
      .style(Style::default().fg(if self.has_focus() {
        Color::Yellow
      } else {
        Color::White
      }));

    let contacts: Vec<ListItem> = self
      .contacts
      .iter()
      .enumerate()
      .map(|(i, contact)| {
        let style = if self.has_focus() && i == self.get_selected() {
          Style::default().fg(Color::Black).bg(Color::White)
        } else {
          Style::default()
        };
        ListItem::new(Line::from(Span::styled(contact, style)))
      })
      .collect();

    let contacts_list = List::new(contacts).block(contacts_block);

    f.render_widget(contacts_list, area);
  }
}
