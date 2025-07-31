use crate::views::{frame::{DrawnView, FocusedView}, ViewContext};
use ratatui::{
  Frame, layout::Rect,
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, Paragraph},
};


#[derive(Clone, Copy, PartialEq)]
pub enum MessageButtonType {
  New,
  Edit,
  Send,
}

pub struct MessageButton {
  button_type: MessageButtonType
}

impl MessageButton {
  pub fn new() -> Self {
    Self { button_type: MessageButtonType::New }
  }

  pub fn next(&mut self) {
    self.button_type = match self.button_type {
      MessageButtonType::New => MessageButtonType::Edit,
      MessageButtonType::Edit => MessageButtonType::Send,
      MessageButtonType::Send => MessageButtonType::New,
    };
  }

  pub fn prev(&mut self) {
    self.button_type = match self.button_type {
      MessageButtonType::New => MessageButtonType::Send,
      MessageButtonType::Send => MessageButtonType::Edit,
      MessageButtonType::Edit => MessageButtonType::New,
    };
  }
}

pub struct ChatMessagePanelView {
  focused: bool,
  current_button: MessageButton
}

impl ChatMessagePanelView {
  pub fn new() -> Self {
    Self { 
      focused: true,
      current_button: MessageButton::new()
    }
  }

  pub fn select_next_button(&mut self) {
    self.current_button.next();
  }

  pub fn select_prev_button(&mut self) {
    self.current_button.prev();
  }

  pub fn process_current_button(&self) {
    
  }

  pub fn current_button_type(&self) -> MessageButtonType {
    self.current_button.button_type
  }
}

impl FocusedView for ChatMessagePanelView {
  fn set_focus(&mut self, set: bool) {
    self.focused = set;
  }

  fn has_focus(&self) -> bool {
    self.focused
  }
}

impl DrawnView for ChatMessagePanelView {
  fn draw(&self, f: &mut Frame, area: Rect, context: &mut dyn ViewContext) {
    let buttons_block = Block::default()
      .borders(Borders::ALL)
      .style(
        Style::default().fg(if self.has_focus() {
          Color::Yellow
        } else {
          Color::White
        }),
      );

    let button_new_style = if self.has_focus() && self.current_button_type() == MessageButtonType::New {
      Style::default().add_modifier(Modifier::REVERSED)
    } else {
      Style::default()
    };
    
    let button_edit_style = if self.has_focus() && self.current_button_type() == MessageButtonType::Edit {
      Style::default().add_modifier(Modifier::REVERSED)
    } else {
      Style::default()
    };

    let button_send_style = if self.has_focus() && self.current_button_type() == MessageButtonType::Send {
      Style::default().add_modifier(Modifier::REVERSED)
    } else {
      Style::default()
    };

    let buttons = vec![Line::from(vec![
      Span::styled(" New ", button_new_style),
      Span::raw(" "),
      Span::styled(" Edit ", button_edit_style),
      Span::raw(" "),
      Span::styled(" Send ", button_send_style),
    ])];

    let buttons_paragraph = Paragraph::new(buttons)
      .block(buttons_block)
      .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(buttons_paragraph, area);
  }
}
