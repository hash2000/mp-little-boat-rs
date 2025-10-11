use std::time::Instant;

use iced::window;

pub mod draft {
    use litle_boat_widgets::widgets::selectable_rich_text;

  #[derive(Debug, Clone)]
  pub enum Link {
    Url(String),
  }

  impl Link {
    pub fn url(&self) -> Option<&String> {
      match self {
        Link::Url(url) => Some(url),
        _ => None,
      }
    }
  }

  impl selectable_rich_text::Link for Link {
    fn underline(&self) -> bool {
        match self {
            Link::Url(_) => true,
        }
    }
}
}

#[derive(Debug, Clone)]
pub enum Message {
  WindowOpened(window::Id),
  Window(window::Id, window::Event),
  Tick(Instant),
  OnLink(draft::Link),
}
