use std::io::Write;

use crossterm::event::{DisableFocusChange, EnableFocusChange};
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use crossterm::{execute, terminal};
use futures_core::Stream;
use ratatui::DefaultTerminal;

pub struct TuiApplication {
}

impl TuiApplication
{
  pub fn new() -> anyhow::Result<Self> {
    let app = Self {  };
    Ok(app)
  }

  pub async fn run(&mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()>
  {

    self.event_loop().await?;
    ratatui::restore();
    Ok(())
  }


  async fn event_loop(&mut self) -> anyhow::Result<()> {


    Ok(())
  }

}
