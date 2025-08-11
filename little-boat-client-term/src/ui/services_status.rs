use ratatui::{
    prelude::*,
    widgets::{*, block::Title},
};

use crate::application::Application;

pub fn draw_services_status(frame: &mut Frame, app: &Application, area: Rect) {
    let block = Block::default()
        .title(Title::from(" Services Status ".to_string()))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let service_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(1, app.services.len() as u32); app.services.len()])
        .split(block.inner(area));

    for (i, service) in app.services.iter().enumerate() {
        let status_color = match service.status.as_str() {
            "Running" => Color::Green,
            "Starting" | "Stopping" => Color::Yellow,
            "Stopped" => Color::Red,
            "Not responding" => Color::Magenta,
            _ => Color::Gray,
        };

        let service_text = format!(
            "{}\nStatus: {}\nLast update: {}s ago",
            service.name,
            service.status,
            service.last_update.elapsed().as_secs()
        );

        let paragraph = Paragraph::new(service_text)
            .style(Style::default().fg(status_color))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(paragraph, service_chunks[i]);
    }

    frame.render_widget(block, area);
}