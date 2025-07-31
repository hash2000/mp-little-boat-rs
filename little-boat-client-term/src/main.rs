mod application;
mod services;
mod views;

use anyhow::Result;
use crossterm::{
  event::{self},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io, time::Duration};
use tokio::sync::mpsc;

use crate::{application::Application, services::{run_services, ServiceEvent}};

#[tokio::main]
async fn main() -> Result<()> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let (tx, mut rx) = mpsc::unbounded_channel::<ServiceEvent>();
  run_services(tx);

  let mut app = Application::new();

  loop {
    if app.exit() {
      break;
    }

    app.begin_frame();

    while let Ok(event) = rx.try_recv() {
      app.handle_service_event(&event);
    }

    terminal.draw(|f| app.draw(f))?;

    if crossterm::event::poll(Duration::from_millis(50))? {
      app.append_event(&event::read()?);
    }
  }

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  Ok(())
}
