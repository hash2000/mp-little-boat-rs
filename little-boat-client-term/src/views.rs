mod frame;
mod chat_view;
mod chat_contacts_view;

use crate::{application::{Application, FocusArea, MessageButton}, views::frame::centered_rect};
pub use crate::views::chat_view::ChatView as ChatView;
pub use crate::views::frame::{
  DrawnView as DrawnView,
  FocusedView as FocusedView
};

use ratatui::{
  Frame, layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::vec;



pub fn ui(f: &mut Frame, app: &Application) {

  app.draw(f);

  /*
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
    .split(f.area());

  let contacts_block = Block::default()
    .title("Контакты")
    .borders(Borders::ALL)
    .style(Style::default().fg(if app.focus == FocusArea::Contacts {
      Color::Yellow
    } else {
      Color::White
    }));

  let contacts: Vec<ListItem> = app
    .contacts
    .iter()
    .enumerate()
    .map(|(i, contact)| {
      let style = if i == app.selected_contact && app.focus == FocusArea::Contacts {
        Style::default().fg(Color::Black).bg(Color::White)
      } else {
        Style::default()
      };
      ListItem::new(Line::from(Span::styled(contact, style)))
    })
    .collect();

  let contacts_list = List::new(contacts).block(contacts_block);

  f.render_widget(contacts_list, chunks[0]);

  // Правая часть (разделена вертикально)
  let right_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
    .split(chunks[1]);

  // Верхняя правая панель - сообщения
  let messages_block = Block::default().title("Сообщения").borders(Borders::ALL);

  let messages_text = app.messages.join("\n");
  let messages_paragraph = Paragraph::new(messages_text).block(messages_block);

  f.render_widget(messages_paragraph, right_chunks[0]);

  // Нижняя правая панель - кнопки
  let buttons_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default().fg(if app.focus == FocusArea::Buttons {
      Color::Yellow
    } else {
      Color::White
    }));

  let button_new_style = if app.focus == FocusArea::Buttons && app.current_button() == MessageButton::New {
    Style::default().add_modifier(Modifier::REVERSED)
  } else {
    Style::default()
  };

  let button_edit_style = if app.focus == FocusArea::Buttons && app.current_button() == MessageButton::Edit
  {
    Style::default().add_modifier(Modifier::REVERSED)
  } else {
    Style::default()
  };

  let button_send_style = if app.focus == FocusArea::Buttons && app.current_button() == MessageButton::Send
  {
    Style::default().add_modifier(Modifier::REVERSED)
  } else {
    Style::default()
  };

  let buttons = vec![Line::from(vec![
    Span::styled(" Новое ", button_new_style),
    Span::raw(" "),
    Span::styled(" Редактировать ", button_edit_style),
    Span::raw(" "),
    Span::styled(" Отправить ", button_send_style),
  ])];

  let buttons_paragraph = Paragraph::new(buttons)
    .block(buttons_block)
    .alignment(ratatui::layout::Alignment::Center);

  f.render_widget(buttons_paragraph, right_chunks[1]);

  
  if app.show_dialog {
    let area = centered_rect(60, 20, f.area());
    let block = Block::default()
      .title("Новое сообщение")
      .borders(Borders::ALL);

    let input = Paragraph::new(app.dialog_input.as_str()).block(block);

    f.render_widget(input, area);

    let buttons_area = Rect {
      x: area.x + area.width - 20,
      y: area.y + area.height - 2,
      width: 18,
      height: 1,
    };

    let ok_style = if app.focus == FocusArea::Dialog {
      Style::default().add_modifier(Modifier::REVERSED)
    } else {
      Style::default()
    };

    let buttons = Paragraph::new(Line::from(vec![
      Span::styled(" OK ", ok_style),
      Span::raw("  "),
      Span::styled(" Cancel ", Style::default()),
    ]));

    f.render_widget(buttons, buttons_area);
    
  }

  */
}

