use anyhow::Result;
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
  Frame, Terminal,
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{io, vec};

// Состояние приложения
struct App {
  contacts: Vec<String>,
  selected_contact: usize,
  selected_button: Button,
  messages: Vec<String>,
  focus: FocusArea,
  show_dialog: bool,
  dialog_input: String,
}

// Области фокусировки
#[derive(PartialEq)]
enum FocusArea {
  Contacts,
  Buttons,
  Dialog,
}

// Кнопки в нижней панели
#[derive(Clone, Copy, PartialEq)]
enum Button {
  New,
  Edit,
  Send,
}

impl Button {
  fn next(&self) -> Self {
    match self {
      Button::New => Button::Edit,
      Button::Edit => Button::Send,
      Button::Send => Button::New,
    }
  }

  fn prev(&self) -> Self {
    match self {
      Button::New => Button::Send,
      Button::Send => Button::Edit,
      Button::Edit => Button::New,
    }
  }
}

impl App {
  fn new() -> Self {
    // Заполняем тестовыми данными
    let contacts = vec![
      "Алексей".to_string(),
      "Мария".to_string(),
      "Иван".to_string(),
      "Ольга".to_string(),
    ];

    let messages = vec![
      "Привет!".to_string(),
      "Как дела?".to_string(),
      "Что нового?".to_string(),
    ];

    App {
      contacts,
      selected_contact: 0,
      selected_button: Button::New,
      messages,
      focus: FocusArea::Contacts,
      show_dialog: false,
      dialog_input: String::new(),
    }
  }

  fn current_button(&self) -> Button {
    self.selected_button
  }
}

pub fn run() -> Result<()> {
  // Настройка терминала
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // Создаем состояние приложения
  let mut app = App::new();

  // Основной цикл приложения
  loop {
    terminal.draw(|f| ui(f, &app))?;

    // Обработка ввода
    if let Event::Key(key) = event::read()? {
      if key.kind == KeyEventKind::Press {
        match key.code {
          KeyCode::Char('q') => break,
          KeyCode::Tab => {
            // Переключение между областями
            app.focus = match app.focus {
              FocusArea::Contacts => FocusArea::Buttons,
              FocusArea::Buttons => FocusArea::Contacts,
              FocusArea::Dialog => FocusArea::Dialog, // В диалоге Tab не работает
            };
          }
          KeyCode::Enter => {
            if app.focus == FocusArea::Buttons && !app.show_dialog {
              if app.current_button() == Button::New {
                app.show_dialog = true;
                app.focus = FocusArea::Dialog;
              }
            } else if app.focus == FocusArea::Dialog {
              // Обработка OK в диалоге
              if !app.dialog_input.is_empty() {
                app.messages.push(app.dialog_input.clone());
                app.dialog_input.clear();
              }
              app.show_dialog = false;
              app.focus = FocusArea::Buttons;
            }
          }
          KeyCode::Right if app.focus == FocusArea::Buttons => {
            app.selected_button = app.selected_button.next()
          }
          KeyCode::Left if app.focus == FocusArea::Buttons => {
            app.selected_button = app.selected_button.prev()
          }
          KeyCode::Up if app.focus == FocusArea::Contacts => {
            if app.selected_contact > 0 {
              app.selected_contact -= 1;
            }
          }
          KeyCode::Down if app.focus == FocusArea::Contacts => {
            if app.selected_contact < app.contacts.len() - 1 {
              app.selected_contact += 1;
            }
          }
          KeyCode::Char(c) if app.focus == FocusArea::Dialog => {
            app.dialog_input.push(c);
          }
          KeyCode::Backspace if app.focus == FocusArea::Dialog => {
            app.dialog_input.pop();
          }
          KeyCode::Esc if app.focus == FocusArea::Dialog => {
            app.show_dialog = false;
            app.focus = FocusArea::Buttons;
          }
          _ => {}
        }
      }
    }
  }

  // Восстановление терминала
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  Ok(())
}

fn ui(f: &mut Frame, app: &App) {
  // Создаем макет
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
    .split(f.size());

  // Левая панель - контакты
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

  let button_new_style = if app.focus == FocusArea::Buttons && app.current_button() == Button::New {
    Style::default().add_modifier(Modifier::REVERSED)
  } else {
    Style::default()
  };

  let button_edit_style = if app.focus == FocusArea::Buttons && app.current_button() == Button::Edit
  {
    Style::default().add_modifier(Modifier::REVERSED)
  } else {
    Style::default()
  };

  let button_send_style = if app.focus == FocusArea::Buttons && app.current_button() == Button::Send
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

  // Диалоговое окно (если нужно)
  if app.show_dialog {
    let area = centered_rect(60, 20, f.size());
    let block = Block::default()
      .title("Новое сообщение")
      .borders(Borders::ALL);

    let input = Paragraph::new(app.dialog_input.as_str()).block(block);

    f.render_widget(input, area);

    // Кнопки OK/Cancel в диалоге
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
}

// Вспомогательная функция для центрирования прямоугольника
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
