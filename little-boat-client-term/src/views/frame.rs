use crate::keymaps::Keymap;

use std::collections::HashSet;

use ratatui::{
  Frame,
  layout::{
    Constraint,
    Direction, 
    Layout,
    Rect
  }
};

use crossterm::event::Event;


pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub trait ViewContext {

}

pub trait FocusedView {
  fn set_focus(&mut self, set: bool);
  fn has_focus(&self) -> bool;
}

pub trait DrawnView {
  fn draw(&self, f: &mut Frame, area: Rect, context: &mut dyn ViewContext);
}

pub trait EventsHandledView {
  fn handle_event(&mut self, event: &Event) -> bool;
}

pub trait View: DrawnView + FocusedView + EventsHandledView { }
impl <T: DrawnView + FocusedView + EventsHandledView> View for T { }
