mod frame;
mod chat_view;
mod chat_contacts_view;
mod chat_messages_view;
mod chat_messages_list_view;
mod chat_messasge_panel_view;
mod chat_view_context;

use crate::application::Application;
pub use crate::views::chat_view::ChatView as ChatView;
pub use crate::views::chat_view_context::ChatViewContext as ChatViewContext;

pub use crate::views::frame::{
  View as View,
  ViewContext as ViewContext,
  DrawnView as DrawnView,
  FocusedView as FocusedView,
};

use ratatui::Frame;



pub fn ui(f: &mut Frame, app: &mut Application) {

  app.draw(f);

  /*


  
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

