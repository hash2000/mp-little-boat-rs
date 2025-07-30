use crate::views::{ChatView, ChatViewContext, View, ViewContext};

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{layout::Rect, Frame};


pub struct Application {
  view: Box<dyn View>,
  view_context: Box<dyn ViewContext>,
  exit: bool
}


impl Application {
  pub fn new() -> Self {
    Application {
      view: Box::new(ChatView::new()), 
      view_context: Box::new(ChatViewContext::new()),
      exit: false,
    }
  }

  pub fn draw(&mut self, f: &mut Frame) {
    self.view.draw(f, Rect::ZERO, &mut *self.view_context);
  }

  pub fn set_view(&mut self, view: Box<dyn View>) {
    self.view = view;
  }

  pub fn begin_frame(&mut self) {
    
    
  }

  pub fn append_event(&mut self, event: &Event) {
    if let Event::Key(key) = event {
      if key.kind == KeyEventKind::Press {
        match key.code {
          KeyCode::Char('q') => if key.modifiers.contains(KeyModifiers::CONTROL) {
            self.exit = true 
          },
          _ => ()
        }
      }
    }

    self.view.handle_event(event);
  }

  pub fn exit(&self) -> bool {
    self.exit
  }
}
