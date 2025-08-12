
use crate::ui::widgets::WidgetRenderer;

use ratatui::{prelude::*, widgets::*};
use crossterm::event::KeyEvent;

pub struct StatusBarWidget {
    pub status: String,
    pub connection_status: String,
}

impl StatusBarWidget {
    pub fn new() -> Self {
        Self {
            status: "Ready".to_string(),
            connection_status: "Connected".to_string(),
        }
    }
    
    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }
    
    pub fn set_connection_status(&mut self, status: String) {
        self.connection_status = status;
    }
}

impl WidgetRenderer for StatusBarWidget {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Status ")
            .border_style(Style::default().fg(Color::Green));
        
        let status_text = format!("{} | {}", self.status, self.connection_status);
        let paragraph = Paragraph::new(status_text)
            .block(block)
            .style(Style::default().fg(Color::White).bg(Color::Blue));
        
        frame.render_widget(paragraph, area);
    }
    
    fn handle_key_event(&mut self, _key: KeyEvent) -> anyhow::Result<bool> {
        Ok(false)
    }
    
    fn get_name(&self) -> &'static str {
        "status"
    }
}
