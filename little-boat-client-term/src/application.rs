use ratatui::{layout::Rect, Frame};

use crate::views::{ChatView, ChatViewContext, View, ViewContext};
use crate::views::DrawnView;

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
    self.view.draw(f, Rect::new(0, 0, 0, 0), &mut *self.view_context);
  }

  pub fn set_context(&mut self, view_context: Box<dyn ViewContext>) {
    self.view_context = view_context;
  }
}
