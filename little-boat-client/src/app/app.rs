use std::time::Duration;

use iced::futures::io::Window;
use iced::window::Id;
use iced::{color, event, window, Element, Font, Subscription, Task, Theme};
use iced::widget::{
  button, center, column, container, mouse_area, opaque, operation, pick_list, rich_text, row, space, span, stack, text, text_input, Column
};
use litle_boat_widgets::widgets::selectable_rich_text::selectable_rich_text;
use litle_boat_widgets::widgets::selectable_text::selectable_text;

use crate::app::Message;
use crate::app::window_events::*;

pub struct App {
  pub(crate) show_modal: bool,
  pub(crate) window_id: window::Id,
}


impl App {
  pub fn new() -> (Self, Task<Message>) {

    let (id, open) = window::open(window::Settings::default());

    (
      Self { 
        show_modal: false, 
        window_id: id,
      },
      open.map(Message::WindowOpened),
    )
  }

  pub fn title(&self, _window_id: window::Id) -> String {
    String::from("Little Boat")
  }

  pub fn theme(&self, _window_id: window::Id) -> Theme {
    Theme::Dark
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
      Message::OnLink(draft) => {
        Task::none()
      }
    }
  }

  pub fn view(&self, id: window::Id) -> Element<'_, Message> {
    // let test_text1 = selectable_rich_text::<_, _, (), _, _>(
    //   vec![
    //     span("Additionally, this tour can also run on WebAssembly "),
    //     span("by leveraging ",),
    //     span("trunk")
    //       .color(color!(0x7777FF))
    //       .underline(true)
    //       .font(Font::MONOSPACE),
    //     span(".")
    //   ])
    //   .on_link(Message::OnLink);

    // let test_text2 = selectable_text("any text");
      
    container(
      row![
        // test_text2,
        // test_text1,
      ]
    ).into()

  }

  fn container(title: &str) -> Column<'_, Message> {
    column![text(title).size(50)].spacing(20)
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
