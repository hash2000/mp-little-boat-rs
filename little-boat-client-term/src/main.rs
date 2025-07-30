mod application;
mod views;

use anyhow::Result;
use crossterm::{
  event::{self},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::io;

use crate::application::Application;


pub fn ui(f: &mut Frame, app: &mut Application) {

  

  /*


  
  if app.show_dialog {
    let area = centered_rect(60, 20, f.area());
    let block = Block::default()
      .title("Новое сообщение")
      .borders(Borders::ALL);

    let input = Paragraph::new(app.dialog_input.as_str()).block(block);

    f.render_widget(input, area);

    let buttons_area = Rect {
      x: area.x + area.width - 20,
      y: area.y + area.height - 2,
      width: 18,
      height: 1,
    };

    let ok_style = if app.focus == FocusArea::Dialog {
      Style::default().add_modifier(Modifier::REVERSED)
    } else {
      Style::default()
    };

    let buttons = Paragraph::new(Line::from(vec![
      Span::styled(" OK ", ok_style),
      Span::raw("  "),
      Span::styled(" Cancel ", Style::default()),
    ]));

    f.render_widget(buttons, buttons_area);
    
  }

  */
}

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
