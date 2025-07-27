use anyhow::Result;
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
  Frame, Terminal,
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{io, vec};


pub struct App {
  pub contacts: Vec<String>,
  pub selected_contact: usize,
  pub selected_button: MessageButton,
  pub messages: Vec<String>,
  pub focus: FocusArea,
  pub show_dialog: bool,
  pub dialog_input: String,
}

#[derive(PartialEq)]
pub enum FocusArea {
  Contacts,
  Buttons,
  Dialog,
}

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
}

impl App {
  pub fn new() -> Self {
    
    let contacts = vec![
      "Алексей".to_string(),
      "Мария".to_string(),
      "Иван".to_string(),
      "Ольга".to_string(),
    ];

    let messages = vec![
      "Привет!".to_string(),
      "Как дела?".to_string(),
      "Что нового?".to_string(),
    ];

    App {
      contacts,
      selected_contact: 0,
      selected_button: MessageButton::New,
      messages,
      focus: FocusArea::Contacts,
      show_dialog: false,
      dialog_input: String::new(),
    }
  }

  pub fn current_button(&self) -> MessageButton {
    self.selected_button
  }
}