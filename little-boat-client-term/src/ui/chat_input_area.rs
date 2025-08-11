use ratatui::{
    prelude::*,
    widgets::{*, block::Title},
};

use crate::application::Application;

pub fn draw_input_area(frame: &mut Frame, _app: &Application, area: Rect) {
    let block = Block::default()
        .title(Title::from(" Input (s: start signaling, c: start chat, x: stop signaling, z: stop chat, q: quit) ".to_string()))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let input_text = "Press keys to control services...";
    let paragraph = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .block(block);

    frame.render_widget(paragraph, area);
}