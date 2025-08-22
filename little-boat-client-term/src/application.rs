use std::io::Write;

use crossterm::event::{DisableFocusChange, EnableFocusChange};
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use crossterm::{execute, terminal};
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
    S: Stream<Item = std::io::Result<crossterm::event::Event>>,
  {
    self.init()?;
    self.event_loop(input).await?;
    self.restore()?;
    Ok(())
  }

  fn init(&mut self) -> anyhow::Result<()> {
    crossterm::style::force_color_output(true);
    enable_raw_mode()?;
    execute!(self.buffer, EnterAlternateScreen, EnableFocusChange)?;
    execute!(self.buffer, terminal::Clear(terminal::ClearType::All))?;
    Ok(())
  }

  fn restore(&mut self) -> anyhow::Result<()> {
    execute!(self.buffer, DisableFocusChange, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
  }

  async fn event_loop<S>(&mut self, input: &mut S) -> anyhow::Result<()>
  where
    S: Stream<Item = std::io::Result<crossterm::event::Event>>,
  {
    Ok(())
  }
}
