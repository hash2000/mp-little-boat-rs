mod chat;

use iced::border;
use iced::keyboard;
use iced::mouse;
use iced::overlay::menu::Catalog;
use iced::overlay::menu::Menu;
use iced::widget;
use iced::widget::text_input;
use iced::widget::{
  button, center, checkbox, column, container, horizontal_rule, horizontal_space, pick_list, row,
  scrollable, stack, text, vertical_rule,
};
use iced::Task;
use iced::{
  Center, Element, Fill, Font, Length, Point, Rectangle, Renderer, Shrink, Subscription, Theme,
  color,
};
use once_cell::sync::Lazy;
use simd_json::derived;

static MESSAGE_LOG: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

#[derive(Debug)]
enum State {
  Disconnected,
  Connected(chat::Connection),
}

fn main() -> iced::Result {
  iced::application("Little Boat (Hash2000)", Program::update, Program::view)
    .subscription(Program::subscription)
    .theme(Program::theme)
    .run_with(Program::new)
}

#[derive(Debug)]
struct Program {
  theme: Theme,
  messages: Vec<chat::Message>,
  new_message: String,
  state: State,
}

#[derive(Debug, Clone)]
enum Message {
    NewMessageChanged(String),
    Send(chat::Message),
    Server,
}

impl Program {

  pub  fn new() -> (Self, Task<Message>) {
    (
      Self {
        theme: Theme::Light,
        messages: vec![
          chat::Message::connected(),
          chat::Message::new("Привет мужик, как делища?").unwrap(),
          chat::Message::new("А сейчас?").unwrap(),
          chat::Message::new("а ещё сейчас вот точно как?").unwrap(),
        ],
        new_message: String::new(),
        state: State::Disconnected,
      },
      Task::batch([
          Task::perform(chat::run_echo(), |_| Message::Server),
          widget::focus_next(),
      ]),
    )
  }

  pub fn theme(&self) -> Theme {
    self.theme.clone()
  }

  pub fn subscription(&self) -> Subscription<Message> {
    use keyboard::key;
    keyboard::on_key_release(|key, modifiers| match key {
      // keyboard::Key::Named(key::Named::ArrowLeft) => {
      //     Some(Message::Previous)
      // }
      // keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
      _ => None,
    })
  }

  pub fn update(&mut self, message: Message) {}

  pub fn view(&self) -> Element<'_, Message> {
    let message_log: Element<_> = if self.messages.is_empty() {
      center(text("Your messages will appear here...").color(color!(0x888888))).into()
    } else {
      scrollable(column(self.messages.iter().map(text).map(Element::from)).spacing(10))
        .id(MESSAGE_LOG.clone())
        .height(Fill)
        .into()
    };

    let new_message_input = {
      let mut input = text_input("Type a message...", &self.new_message)
        .on_input(Message::NewMessageChanged)
        .padding(10);

      let mut button = button(text("Send")
          .height(40)
          .align_y(Center))
        .padding([0, 20]);

      if matches!(self.state, State::Connected(_)) {
        if let Some(message) = chat::Message::new(&self.new_message) {
          input = input.on_submit(Message::Send(message.clone()));
          button = button.on_press(Message::Send(message));
        }
      }

      row![input, button]
        .spacing(10)
        .align_y(Center)
    };

    column![message_log, new_message_input]
      .height(Fill)
      .padding(20)
      .spacing(10)
      .into()

  }
}
