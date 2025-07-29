mod application;
mod views;

use anyhow::Result;
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crate::application::Application;

pub fn main() -> Result<()> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app = Application::new();

  loop {
    terminal.draw(|f| views::ui(f, &mut app))?;

    if let Event::Key(key) = event::read()? {
      if key.kind == KeyEventKind::Press {
        match key.code {
          KeyCode::Char('q') => break,
          _ => {}
        }
      }
    }
  }

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  Ok(())
}
