mod frame;
mod chat_board_view;
mod chat_board_events;
mod chat_contacts_view;
mod chat_contacts_events;
mod chat_messages_view;
mod chat_messages_events;
mod chat_messages_list_view;
mod chat_messages_list_events;
mod chat_messasge_panel_view;
mod chat_messasge_panel_events;
mod chat_view_context;

pub use crate::views::chat_board_view::ChatBoardView as ChatBoardView;
pub use crate::views::chat_view_context::ChatViewContext as ChatViewContext;

pub use crate::views::frame::{
  View as View,
  ViewContext as ViewContext,
  DrawnView as DrawnView,
  FocusedView as FocusedView,
  EventsHandledView as EventsHandledView,
};

