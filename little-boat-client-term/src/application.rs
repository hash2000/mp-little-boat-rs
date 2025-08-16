use std::io::Write;

use crossterm::event::EnableFocusChange;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use futures_core::Stream;

pub struct TuiApplication<W>
where
  W: Write,
{
  buffer: W,
}

impl<W> TuiApplication<W>
where
  W: Write,
{
  pub fn new(buffer: W) -> anyhow::Result<Self> {
    let app = Self { buffer };
    Ok(app)
  }

  pub async fn run<S>(&mut self, input: &mut S) -> anyhow::Result<()>
  where
    S: Stream<Item = crossterm::event::Event>,
  {
    crossterm::style::force_color_output(true);
    enable_raw_mode()?;
    execute!(self.buffer, EnterAlternateScreen, EnableFocusChange)?;

    self.prepare_frame().await?;

    Ok(())
  }

  async fn prepare_frame(&mut self) -> anyhow::Result<()> {
    Ok(())
  }
}
