mod frame;
mod chat;


pub use crate::views::chat::ChatBoardView as ChatBoardView;
pub use crate::views::chat::ChatViewContext as ChatViewContext;

pub use crate::views::frame::{
  View as View,
  ViewContext as ViewContext,
  DrawnView as DrawnView,
  FocusedView as FocusedView,
  EventsHandledView as EventsHandledView,
};