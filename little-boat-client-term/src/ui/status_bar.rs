use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::application::{AppState, Application};

pub fn draw_status_bar(frame: &mut Frame, app: &Application, area: Rect) {
    let status_text = match app.state {
        AppState::Initializing => "Initializing...",
        AppState::Running => "Running",
        AppState::Shutdown => "Shutting down...",
    };

    let connected_users = if app.connected_users.is_empty() {
        "No users connected".to_string()
    } else {
        format!("Connected users: {}", app.connected_users.len())
    };

    let right_text = format!("{} | {}", connected_users, app.status_message);

    let paragraph = Paragraph::new(Line::from(vec![
        Span::styled(status_text, Style::default().fg(Color::Green)),
        Span::raw(" | "),
        Span::styled(right_text, Style::default().fg(Color::Gray)),
    ]))
    .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_widget(paragraph, area);
}