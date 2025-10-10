use iced::{window::Event, Task};

use crate::app::{App, Message};


impl App {
  pub(crate) fn process_window_events(&self, event: Event) -> Task<Message> {
    match event {
      Event::Closed => {
        iced::exit()
      },
      Event::CloseRequested => {
        iced::exit()
      },
      Event::Opened { position, size } => {
        Task::none()
      },
      _ => Task::none()
    }
  }
}
