use crate::{keymaps::Keymap, views::{frame::ViewContext, View}};

use std::collections::HashSet;
use crossterm::event::Event;

pub struct ChatViewContext {
  keymap: HashSet<Keymap>
}

impl ChatViewContext {
  pub fn new() -> Self {
    Self { 
      keymap: HashSet::new()
    }
  }
}

impl ViewContext for ChatViewContext {

}