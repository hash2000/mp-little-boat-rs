pub mod dashboard_id;


use iced::{alignment, window, Element, Theme};
use iced::widget::{button, center, container, row, text};

use crate::app::Message;


#[derive(Debug, Clone)]
pub struct Dashboard {
  title: String,

}


#[derive(Debug, Clone)]
pub enum Event {
  
}

impl Dashboard {
  pub fn new(options: String) -> Self {
    Self {
      title: "title".to_string(),
      
    }
  }

  pub fn update(&mut self, message: Message) {

  }

  pub fn view<'a>(&'a self, id: window::Id, theme: &Theme) -> Element<'a, Message> {
    let content = row![
        container(text("Test"))
        .align_x(alignment::Horizontal::Center)
        .width(250)
    ];

    center(content).into()
  }
}
