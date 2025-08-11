use ratatui::{
    prelude::*,
    widgets::{*, block::Title},
};

use crate::application::Application;

pub fn draw_chat_area(frame: &mut Frame, app: &Application, area: Rect) {
    let block = Block::default()
        .title(Title::from(" Chat Messages ".to_string()))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    // Создаем список сообщений
    let messages: Vec<ListItem> = app.chat_messages
        .iter()
        .map(|msg| {
            let content = Line::from(msg.clone());
            ListItem::new(content)
        })
        .collect();

    let list = List::new(messages)
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(list, area);
}