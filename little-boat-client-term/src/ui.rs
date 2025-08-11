mod chat_area;
mod chat_input_area;
mod services_status;
mod status_bar;

use ratatui::{
  prelude::*,
  widgets::{block::Title, *},
};

use crate::{
  application::{AppState, Application},
  ui::{
    chat_area::draw_chat_area, chat_input_area::draw_input_area,
    services_status::draw_services_status, status_bar::draw_status_bar,
  },
};

pub fn draw(frame: &mut Frame, app: &Application) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Length(3), // Header
      Constraint::Length(8), // Services status
      Constraint::Min(10),   // Chat area
      Constraint::Length(3), // Input area
      Constraint::Length(1), // Status bar
    ])
    .split(frame.size());

  // Header
  let header = Paragraph::new("Little Boat P2P Chat")
    .style(Style::default().fg(Color::White).bg(Color::Blue))
    .alignment(Alignment::Center);
  frame.render_widget(header, chunks[0]);

  // Services status
  draw_services_status(frame, app, chunks[1]);

  // Chat area
  draw_chat_area(frame, app, chunks[2]);

  // Input area
  draw_input_area(frame, app, chunks[3]);

  // Status bar
  draw_status_bar(frame, app, chunks[4]);
}
