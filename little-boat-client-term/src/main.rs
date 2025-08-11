mod application;
mod ui;

use anyhow::Result;
use application::poll_event;
use crossterm::{
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self};
use tokio::sync::mpsc;

use crate::application::Application;

#[tokio::main]
async fn main() -> Result<()> {
  // initialize terminal
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;

  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // create Application
  let mut app = Application::new();

  // create application event tssk
  let (event_tx, mut event_rx) = mpsc::unbounded_channel();
  tokio::spawn(async move {
    loop {
      if poll_event(event_tx.clone()).await.is_err() {
        break;
      }
    }
  });

  // run main loop
  app.run(&mut terminal, &mut event_rx).await?;

  // end TUI
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

  Ok(())
}
