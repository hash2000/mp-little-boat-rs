use crate::{keymaps::Keymap, views::frame::ViewContext};

use std::collections::HashSet;

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