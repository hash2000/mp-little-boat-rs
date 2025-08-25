use iced::border;
use iced::keyboard;
use iced::mouse;
use iced::overlay::menu::Menu;
use iced::widget::{
  button, center, checkbox, column, container, horizontal_rule, horizontal_space,
  pick_list, row, scrollable, stack, text, vertical_rule,
};
use iced::{
  Center, Element, Fill, Font, Length, Point, Rectangle, Renderer, Shrink, Subscription, Theme,
  color,
};
use simd_json::derived;

fn main() -> iced::Result {
  iced::application(Program::title, Program::update, Program::view)
    .subscription(Program::subscription)
    .theme(Program::theme)
    .run()
}

#[derive(Debug)]
struct Program {
  title: &'static str,
  theme: Theme,
}

impl Default for Program {
    fn default() -> Self {
        Self { title: "Little Boat (Hash2000)", theme: Theme::default() }
    }
}

#[derive(Debug, Clone)]
enum Message {
  Initialized,
}

impl Program {
  pub fn title(&self) -> String {
    self.title.to_string()
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
    let header = row![
      text(self.title).size(20).font(Font::MONOSPACE),
      horizontal_space(),
    ]
    .spacing(20)
    .align_y(Center);

    // let example = center(if self.explain {
    //   self.example.view().explain(color!(0x0000ff))
    // } else {
    //   self.example.view()
    // })
    // .style(|theme| {
    //   let palette = theme.extended_palette();

    //   container::Style::default().border(border::color(palette.background.strong.color).width(4))
    // })
    // .padding(4);

    // let controls = row(
    //   [
    //     (!self.example.is_first())
    //       .then_some(button("← Previous").padding([5, 10]).on_press(Message::Previous).into()),
    //     Some(horizontal_space().into()),
    //     (!self.example.is_last())
    //       .then_some(button("Next →").padding([5, 10]).on_press(Message::Next).into()),
    //   ]
    //   .into_iter()
    //   .flatten(),
    // );

    column![header].spacing(10).padding(20).into()
  }
}
