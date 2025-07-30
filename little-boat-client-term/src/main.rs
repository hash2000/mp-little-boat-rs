mod application;
mod views;
mod keymaps;

use anyhow::Result;
use crossterm::{
  event::{self},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
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

    if app.exit() {
      break;
    }

    terminal.draw(|f| app.draw(f))?;

    app.begin_frame();
    app.append_event(&event::read()?);
  }

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  Ok(())
}
