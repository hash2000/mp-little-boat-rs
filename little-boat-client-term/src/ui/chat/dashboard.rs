use crate::ui::widgets::WidgetRenderer;

use ratatui::{prelude::*, widgets::*};
use crossterm::event::KeyEvent;


pub struct ChatDashboardWidget {
    pub messages: Vec<String>,
}

impl ChatDashboardWidget {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    
    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
        // Messages limit
        if self.messages.len() > 1000 {
            self.messages.remove(0);
        }
    }
}

impl WidgetRenderer for ChatDashboardWidget {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Chat ")
            .border_style(Style::default().fg(Color::Cyan));
        
        // Messages list
        let messages: Vec<ListItem> = self.messages
            .iter()
            .map(|msg| ListItem::new(msg.clone()))
            .collect();
        
        let list = List::new(messages)
            .block(block)
            .style(Style::default().fg(Color::White));
        
        frame.render_widget(list, area);
    }
    
    fn handle_key_event(&mut self, _key: KeyEvent) -> anyhow::Result<bool> {
        Ok(false)
    }
    
    fn get_name(&self) -> &'static str {
        "chat"
    }
}
