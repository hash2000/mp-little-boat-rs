use crate::views::{frame::{DrawnView, EventsHandledView, FocusedView}, ViewContext};
use crossterm::event::Event;
use ratatui::{
  Frame, layout::Rect,
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, Paragraph},
};


#[derive(Clone, Copy, PartialEq)]
pub enum MessageButton {
  New,
  Edit,
  Send,
}

impl MessageButton {
  pub fn next(&self) -> Self {
    match self {
      MessageButton::New => MessageButton::Edit,
      MessageButton::Edit => MessageButton::Send,
      MessageButton::Send => MessageButton::New,
    }
  }

  pub fn prev(&self) -> Self {
    match self {
      MessageButton::New => MessageButton::Send,
      MessageButton::Send => MessageButton::Edit,
      MessageButton::Edit => MessageButton::New,
    }
  }

  pub fn style(button: MessageButton, view: &ChatMessagePanelView) -> Style{    
    let style =  if view.has_focus() && view.current_button() == button {
        Style::default().add_modifier(Modifier::REVERSED)
      } else {
        Style::default()
      };

      style
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
      current_button: MessageButton::New
    }
  }

  pub fn current_button(&self) -> MessageButton {
    self.current_button
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

    let button_new_style = MessageButton::style(MessageButton::New, self);
    let button_edit_style = MessageButton::style(MessageButton::Edit, self);
    let button_send_style = MessageButton::style(MessageButton::Send, self);

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
