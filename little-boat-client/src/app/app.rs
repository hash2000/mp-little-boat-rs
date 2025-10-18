use std::time::Duration;

use iced::{color, padding, window, Element, Font, Length, Subscription, Task, Theme};
use iced::widget::{
  button, center, column, container, mouse_area, opaque, operation, pick_list, rich_text, row, 
  space, span, stack, text, text_input, Column
};

use crate::app::Message;

pub struct App {
  pub(crate) window_id: window::Id,
}

impl App {
  pub fn new() -> (Self, Task<Message>) {

    let (id, open) = window::open(window::Settings::default());

    (
      Self {
        window_id: id,
      },
      open.map(Message::WindowOpened),
    )
  }

  pub fn title(&self, _window_id: window::Id) -> String {
    String::from("Little Boat")
  }

  pub fn theme(&self, _window_id: window::Id) -> Theme {
    Theme::KanagawaDragon
  }

  pub fn scale_factor(&self, _window_id: window::Id) -> f32 {
    1.0
  }


  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::WindowOpened(id) => {
          Task::none()
        },
        Message::Window(id, event ) => {
          if id == self.window_id {
            self.process_window_events(event)
          } else {
            Task::none()
          }
        },
        Message::Tick(now) => {
          Task::none()
        },
    }
  }
  
  pub fn view(&self, id: window::Id) -> Element<'_, Message> {
    if id != self.window_id {
      column![].into()
    } else {
      // let screen = match &self.screen {
      //   Screen::Welcome(welcome) => welcome.view(id).map(Message::Welcome)
      // };

      // let content = container(
      // container(screen)
      //   .width(Length::Fill)
      //   .height(Length::Fill)
      //   .style(theme::container::general),
      // )
      // .padding(padding::top(0));

      column![].into()
    }
  }

  pub fn subscription(&self) -> Subscription<Message> {
    let tick = iced::time::every(Duration::from_secs(1)).map(Message::Tick);
    
    let subscriptions = vec![
      window::events()
        .map(|(window, event)| Message::Window(window, event)),
      tick
    ];

    Subscription::batch(subscriptions) 
  }
}
