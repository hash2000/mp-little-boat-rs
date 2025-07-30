use crate::views::{ChatView, ChatViewContext, View, ViewContext};
use crate::views::DrawnView;

use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};


pub struct Application {
  view: Box<dyn View>,
  view_context: Box<dyn ViewContext>,
}


impl Application {
  pub fn new() -> Self {
    Application {
      view: Box::new(ChatView::new()), 
      view_context: Box::new(ChatViewContext::new())
    }
  }

  pub fn draw(&mut self, f: &mut Frame) {
    self.view.draw(f, Rect::ZERO, &mut *self.view_context);
  }

  pub fn set_view(&mut self, view: Box<dyn View>) {
    self.view = view;
  }

  pub fn set_context(&mut self, view_context: Box<dyn ViewContext>) {
    self.view_context = view_context;
  }

  pub fn begin_frame(&mut self) {
    self.view_context.begin_frame();
  }

  pub fn append_event(&mut self, event: &Event) {
    self.view_context.append_event(event);
  }

  pub fn exit(&self) -> bool {
    self.view_context.exit()
  }
}
