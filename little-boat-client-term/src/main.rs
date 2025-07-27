mod views;
mod app;

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

use crate::app::{App, MessageButton, FocusArea};


pub fn main() -> Result<()> {

  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app = App::new();


  loop {
    terminal.draw(|f| views::ui(f, &app))?;

    if let Event::Key(key) = event::read()? {
      if key.kind == KeyEventKind::Press {
        match key.code {
          KeyCode::Char('q') => break,
          KeyCode::Tab => {
            app.focus = match app.focus {
              FocusArea::Contacts => FocusArea::Buttons,
              FocusArea::Buttons => FocusArea::Contacts,
              FocusArea::Dialog => FocusArea::Dialog,
            };
          }
          KeyCode::Enter => {
            if app.focus == FocusArea::Buttons && !app.show_dialog {
              if app.current_button() == MessageButton::New {
                app.show_dialog = true;
                app.focus = FocusArea::Dialog;
              }
            } else if app.focus == FocusArea::Dialog {
              if !app.dialog_input.is_empty() {
                app.messages.push(app.dialog_input.clone());
                app.dialog_input.clear();
              }
              app.show_dialog = false;
              app.focus = FocusArea::Buttons;
            }
          }
          KeyCode::Right if app.focus == FocusArea::Buttons => {
            app.selected_button = app.selected_button.next()
          }
          KeyCode::Left if app.focus == FocusArea::Buttons => {
            app.selected_button = app.selected_button.prev()
          }
          KeyCode::Up if app.focus == FocusArea::Contacts => {
            if app.selected_contact > 0 {
              app.selected_contact -= 1;
            }
          }
          KeyCode::Down if app.focus == FocusArea::Contacts => {
            if app.selected_contact < app.contacts.len() - 1 {
              app.selected_contact += 1;
            }
          }
          KeyCode::Char(c) if app.focus == FocusArea::Dialog => {
            app.dialog_input.push(c);
          }
          KeyCode::Backspace if app.focus == FocusArea::Dialog => {
            app.dialog_input.pop();
          }
          KeyCode::Esc if app.focus == FocusArea::Dialog => {
            app.show_dialog = false;
            app.focus = FocusArea::Buttons;
          }
          _ => {}
        }
      }
    }
  }

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  Ok(())
}
