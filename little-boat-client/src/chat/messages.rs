use std::fmt;
use iced::widget::text;

#[derive(Debug, Clone)]
pub enum Message {
  Connected,
  Disconnected,
  User(String),
}

impl Message {
  pub fn new(message: &str) -> Option<Self> {
    if message.is_empty() {
      None
    } else {
      Some(Self::User(message.to_string()))
    }
  }

  pub fn connected() -> Self {
    Message::Connected
  }

  pub fn disconnected() -> Self {
    Message::Disconnected
  } 

  pub fn as_str(&self) -> &str {
    match self {
      Message::Connected => "Connected successfully!",
      Message::Disconnected => "Connection lost... Retrying...",
      Message::User(message) => message.as_str(),
    }
  }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> text::IntoFragment<'a> for &'a Message {
    fn into_fragment(self) -> text::Fragment<'a> {
        text::Fragment::Borrowed(self.as_str())
    }
}