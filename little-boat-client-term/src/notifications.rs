use std::time::{Duration, Instant};
use ratatui::style::{Style, Color};

// Структура для нотификации
#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
    pub timestamp: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationLevel {
    Info,
    Warn,
    Error,
    Debug,
}

impl Notification {
    pub fn new(message: String, level: NotificationLevel) -> Self {
        Self {
            message,
            level,
            timestamp: Instant::now(),
        }
    }
    
    pub fn is_expired(&self, duration: Duration) -> bool {
        self.timestamp.elapsed() > duration
    }
    
    pub fn style(&self) -> Style {
        match self.level {
            NotificationLevel::Info => Style::default().fg(Color::Blue),
            NotificationLevel::Warn => Style::default().fg(Color::Yellow),
            NotificationLevel::Error => Style::default().fg(Color::Red),
            NotificationLevel::Debug => Style::default().fg(Color::Gray),
        }
    }
}

pub struct NotificationManager {
    notifications: Vec<Notification>,
    max_notifications: usize,
    display_duration: Duration,
}

impl NotificationManager {
    pub fn new(max_notifications: usize, display_duration: Duration) -> Self {
        Self {
            notifications: Vec::new(),
            max_notifications,
            display_duration,
        }
    }
    
    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
        // Notifications limit
        if self.notifications.len() > self.max_notifications {
            self.notifications.remove(0);
        }
    }
    
    pub fn cleanup_expired(&mut self) {
        self.notifications.retain(|n| !n.is_expired(self.display_duration));
    }
    
    pub fn get_visible_notifications(&self) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| !n.is_expired(self.display_duration))
            .collect()
    }
    
    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }
    
    pub fn get_count(&self) -> usize {
        self.notifications.len()
    }
}