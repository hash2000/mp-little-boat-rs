mod application;
mod views;

use anyhow::Result;
use crossterm::{
  event::{self},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use little_boat_abstractions::ServiceEvent;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io, time::Duration};
use tokio::sync::mpsc;

use crate::application::Application;

#[tokio::main]
async fn main() -> Result<()> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;


  let mut app = Application::new()?;
  app.init().await?;

  loop {
    if app.exit() {
      break;
    }

    app.begin_frame();

    // while let Ok(event) = rx.try_recv() {
    //   app.handle_service_event(&event);
    // }

    terminal.draw(|f| app.draw(f))?;

    if crossterm::event::poll(Duration::from_millis(50))? {
      app.append_event(&event::read()?);
    }
  }

  app.shutdown().await?;

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  Ok(())
}
