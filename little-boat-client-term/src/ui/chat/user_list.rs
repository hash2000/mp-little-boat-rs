use crate::ui::widgets::WidgetRenderer;

use ratatui::{prelude::*, widgets::*};
use crossterm::event::{KeyEvent, KeyCode};

pub struct UserListWidget {
    pub users: Vec<String>,
    pub selected_index: usize,
}

impl UserListWidget {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            selected_index: 0,
        }
    }
    
    pub fn add_user(&mut self, user: String) {
        self.users.push(user);
    }
    
    pub fn remove_user(&mut self, user: &str) {
        self.users.retain(|u| u != user);
        if self.selected_index >= self.users.len() && !self.users.is_empty() {
            self.selected_index = self.users.len() - 1;
        }
    }
}

impl WidgetRenderer for UserListWidget {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Users ")
            .border_style(Style::default().fg(Color::Magenta));
        
        let items: Vec<ListItem> = self.users
            .iter()
            .enumerate()
            .map(|(i, user)| {
                let style = if i == self.selected_index {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(user.clone()).style(style)
            })
            .collect();
        
        let list = List::new(items)
            .block(block)
            .style(Style::default().fg(Color::White));
        
        frame.render_widget(list, area);
    }
    
    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<bool> {
        match key.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                Ok(true)
            }
            KeyCode::Down => {
                if self.selected_index < self.users.len().saturating_sub(1) {
                    self.selected_index += 1;
                }
                Ok(true)
            }
            _ => Ok(false)
        }
    }
    
    fn get_name(&self) -> &'static str {
        "users"
    }
}