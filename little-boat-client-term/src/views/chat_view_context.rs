use crate::views::frame::ViewContext;

use crossterm::event::Event;

pub struct ChatViewContext {

}

impl ChatViewContext {
  pub fn new() -> Self {
    Self {  }
  }
}

impl ViewContext for ChatViewContext {
  fn begin_frame(&mut self) {

  }

  fn append_event(&mut self, event: &Event) {

  }
  
  fn exit(&self) -> bool {
    false
  }
}