use crate::ui::widgets::WidgetRenderer;

use ratatui::{prelude::*, widgets::*};
use crossterm::event::{KeyEvent, KeyCode};

pub struct InputWidget {
    pub buffer: String,
    pub on_submit: Option<Box<dyn Fn(&str) -> bool>>, // true если нужно очистить буфер
}

impl InputWidget {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            on_submit: None,
        }
    }
    
    pub fn set_on_submit<F>(&mut self, callback: F) 
    where 
        F: Fn(&str) -> bool + 'static,
    {
        self.on_submit = Some(Box::new(callback));
    }
}

impl WidgetRenderer for InputWidget {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Input ")
            .border_style(Style::default().fg(Color::Cyan));
        
        let input = Paragraph::new(self.buffer.as_str())
            .block(block)
            .style(Style::default().fg(Color::Yellow));
        
        frame.render_widget(input, area);
        
        frame.set_cursor_position(Position {
          x: area.x + self.buffer.len() as u16 + 1,
          y: area.y + 1,
        });
    }
    
    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<bool> {
        match key.code {
            KeyCode::Char(c) => {
                self.buffer.push(c);
                Ok(true)
            }
            KeyCode::Backspace => {
                self.buffer.pop();
                Ok(true)
            }
            KeyCode::Enter => {
                if !self.buffer.is_empty() {
                    let should_clear = if let Some(callback) = &self.on_submit {
                        callback(&self.buffer)
                    } else {
                        true
                    };
                    
                    if should_clear {
                        self.buffer.clear();
                    }
                }
                Ok(true)
            }
            KeyCode::Esc => {
                self.buffer.clear();
                Ok(true)
            }
            _ => Ok(false)
        }
    }
    
    fn get_name(&self) -> &'static str {
        "input"
    }
}
