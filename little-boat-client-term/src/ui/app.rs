use std::io::{self, Stdout, stdout};
use std::time::{Duration, Instant};

use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use crossterm::terminal::{
  EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::notifications::{Notification, NotificationLevel, NotificationManager};
use crate::ui::chat;
use crate::ui::widgets::WidgetRenderer;

pub struct TuiApp {
  pub notification_manager: NotificationManager,
  pub show_notifications_dialog: bool,
  pub widgets: Vec<Box<dyn WidgetRenderer>>,
  pub active_widget_index: usize,
  pub should_quit: bool,
}

impl TuiApp {
  pub fn new() -> io::Result<Self> {
    let mut app = Self {
      notification_manager: NotificationManager::new(10, Duration::from_secs(10)),
      show_notifications_dialog: false,
      widgets: Vec::new(),
      active_widget_index: 0,
      should_quit: false,
    };

    app.add_widget(Box::new(chat::ChatDashboardWidget::new()));
    app.add_widget(Box::new(chat::InputWidget::new()));
    app.add_widget(Box::new(chat::StatusBarWidget::new()));
    app.add_widget(Box::new(chat::UserListWidget::new()));

    Ok(app)
  }

  pub fn add_widget(&mut self, widget: Box<dyn WidgetRenderer>) {
    self.widgets.push(widget);
  }

  pub fn add_notification(&mut self, message: String, level: NotificationLevel) {
    self.notification_manager.add_notification(Notification::new(message, level));
  }

  pub fn cleanup_notifications(&mut self) {
    self.notification_manager.cleanup_expired();
  }

  pub fn should_quit(&self) -> bool {
    self.should_quit
  }

  pub fn render(&self, frame: &mut Frame) {
    let size = frame.area();

    // Основной UI
    self.render_main_ui(frame, size);

    if self.show_notifications_dialog {
      self.render_notifications_dialog(frame, size);
    } else {
      self.render_notifications(frame, size);
    }
  }

  fn render_main_ui(&self, frame: &mut Frame, area: Rect) {
    // Создаем основной layout
    let chunks = ratatui::layout::Layout::default()
      .direction(ratatui::layout::Direction::Vertical)
      .constraints([
        ratatui::layout::Constraint::Min(3),    // Чат
        ratatui::layout::Constraint::Length(3), // Input
      ])
      .split(area);

    // Рендерим виджеты
    if self.widgets.len() >= 2 {
      // Рендерим чат (первый виджет)
      self.widgets[0].render(frame, chunks[0]);
      // Рендерим input (второй виджет)
      self.widgets[1].render(frame, chunks[1]);
    }
  }

  fn render_notifications(&self, frame: &mut Frame, area: Rect) {
    let notifications = self.notification_manager.get_visible_notifications();
    if notifications.is_empty() {
      return;
    }

    // Вычисляем размеры для нотификаций
    let max_width = 50u16.min(area.width.saturating_sub(2));
    let max_height = 10u16.min(area.height.saturating_sub(2));

    // Создаем область для нотификаций в правом нижнем углу
    let notification_area = Rect {
      x: area.x + area.width.saturating_sub(max_width + 2),
      y: area.y + area.height.saturating_sub(max_height + 2),
      width: max_width.min(area.width.saturating_sub(2)),
      height: max_height.min(area.height.saturating_sub(2)),
    };

    // Создаем блок для нотификаций с полупрозрачным фоном
    let block = Block::default()
      .borders(Borders::ALL)
      .border_style(Style::default().fg(Color::DarkGray))
      .style(Style::default().bg(Color::Black));

    // Создаем список нотификаций
    let items: Vec<ListItem> = notifications
      .iter()
      .rev() // Показываем последние сверху
      .take(5) // Ограничиваем количество
      .map(|notification| {
        let content = format!(
          "[{}] {}",
          match notification.level {
            NotificationLevel::Info => "INFO",
            NotificationLevel::Warn => "WARN",
            NotificationLevel::Error => "ERROR",
            NotificationLevel::Debug => "DEBUG",
          },
          &notification.message
        );
        ListItem::new(content).style(notification.style())
      })
      .collect();

    let list =
      List::new(items).block(block).style(Style::default().bg(Color::Black).fg(Color::White));

    frame.render_widget(Clear, notification_area); // Очищаем область
    frame.render_widget(list, notification_area);
  }

  fn render_notifications_dialog(&self, frame: &mut Frame, area: Rect) {
    // Создаем затемненный фон
    let overlay_area = Rect {
      x: area.x + 2,
      y: area.y + 2,
      width: area.width.saturating_sub(4).max(1),
      height: area.height.saturating_sub(4).max(1),
    };

    // Блок диалога
    let block = Block::default()
      .borders(Borders::ALL)
      .title(" Notifications (Press 'q' or 'Esc' to close) ")
      .style(Style::default().bg(Color::Blue).fg(Color::White));

    // Получаем все нотификации
    let notifications = self.notification_manager.get_visible_notifications();
    let items: Vec<ListItem> = notifications
      .iter()
      .rev()
      .map(|notification| {
        let timestamp = notification.timestamp.elapsed().as_secs();
        let content = format!(
          "[{}s] [{}] {}",
          timestamp,
          match notification.level {
            NotificationLevel::Info => "INFO",
            NotificationLevel::Warn => "WARN",
            NotificationLevel::Error => "ERROR",
            NotificationLevel::Debug => "DEBUG",
          },
          &notification.message
        );
        ListItem::new(content).style(notification.style())
      })
      .collect();

    let list =
      List::new(items).block(block).style(Style::default().bg(Color::Black).fg(Color::White));

    frame.render_widget(Clear, overlay_area);
    frame.render_widget(list, overlay_area);
  }

  pub fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
    if self.show_notifications_dialog {
      match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
          self.show_notifications_dialog = false;
        }
        _ => {}
      }
      return Ok(());
    }

    if key.modifiers == KeyModifiers::CONTROL {
      match key.code {
        KeyCode::Char('l') | KeyCode::Char('L') => {
          self.show_notifications_dialog = true;
          return Ok(());
        }
        KeyCode::Char('c') => {
          self.should_quit = true;
          return Ok(());
        }
        _ => {}
      }
    }

    // Передаем событие активному виджету
    if !self.widgets.is_empty() {
      let active_widget = &mut self.widgets[self.active_widget_index];
      if active_widget.handle_key_event(key)? {
        return Ok(());
      }
    }

    Ok(())
  }
}
