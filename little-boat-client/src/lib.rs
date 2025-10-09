use iced::event::{self, Event};
use iced::{keyboard, window};
use iced::keyboard::key;
use iced::widget::{
  button, center, column, container, mouse_area, opaque, operation, pick_list, row, space, stack,
  text, text_input,
};
use iced::{Bottom, Color, Element, Fill, Subscription, Task};

use std::fmt;

pub fn run_app() -> anyhow::Result<()> {

  iced::daemon(App::default, App::update, App::view)
    .subscription(App::subscription)
    .title()
    .run()?;

  Ok(())
}

#[derive(Default)]
struct App {
  show_modal: bool,
}

#[derive(Debug, Clone)]
enum Message {
  Event(Event),
}

impl App {
  fn subscription(&self) -> Subscription<Message> {
    event::listen().map(Message::Event)
  }

  fn title(&self, _window_id: window::Id) -> String {
    String::from("Little Boat")
  }

  fn update(&mut self, message: Message) -> Task<Message> {
    Task::none()
  }

  fn view(&self, id: window::Id) -> Element<'_, Message> {
    space().into()
  }
}

impl App {
  fn hide_modal(&mut self) {
    self.show_modal = false;
  }
}

fn modal<'a, Message>(
  base: impl Into<Element<'a, Message>>,
  content: impl Into<Element<'a, Message>>,
  on_blur: Message,
) -> Element<'a, Message>
where
  Message: Clone + 'a,
{
  stack![
    base.into(),
    opaque(
      mouse_area(center(opaque(content)).style(|_theme| {
        container::Style {
          background: Some(Color { a: 0.8, ..Color::BLACK }.into()),
          ..container::Style::default()
        }
      }))
      .on_press(on_blur)
    )
  ]
  .into()
}
