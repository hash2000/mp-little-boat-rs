use cstr::cstr;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use qmetaobject::*;
use std::{collections::HashMap, ops::Index};

// chat message types
#[derive(Debug, Clone)]
pub enum MessageItemType {
  Text,
  Header1,
  Header2,
  Header3,
  Code,
  Quote,
}

#[derive(Debug, Clone)]
pub struct MessageItem {
  pub content_type: MessageItemType,
  pub content: String,
}

impl From<MessageItemType> for QVariant {
  fn from(item_type: MessageItemType) -> Self {
    (item_type as i32).into()
  }
}

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct ChatMessage {
  base: qt_base_class!(trait QAbstractListModel),
  count: qt_property!(i32; READ row_count NOTIFY count_changed),
  count_changed: qt_signal!(),
  list: Vec<MessageItem>,
}

impl ChatMessage {
  pub fn push(&mut self, item: MessageItem) {
    let end = self.list.len();
    (self as &mut dyn QAbstractListModel).begin_insert_rows(end as i32, end as i32);
    self.list.insert(end, item);
    (self as &mut dyn QAbstractListModel).end_insert_rows();
    self.count_changed();
  }
} 

impl From<String> for ChatMessage {
  fn from(msg: String) -> Self { 
    let mut message = ChatMessage::default();     
    let items = parse_markdown_impl(msg);
    for item in items {
      message.push(item);
    }
    message
  }
}

impl QAbstractListModel for ChatMessage {
  fn row_count(&self) -> i32 {
    self.list.len() as i32
  }
  
  fn data(&self, index: QModelIndex, role: i32) -> QVariant {
    let idx = index.row() as usize;
    if idx < self.list.len() {
      if role == USER_ROLE {
        QVariant::from(self.list[idx].content_type.clone())
      } else if role == USER_ROLE + 1 {
        QString::from(self.list[idx].content.clone()).into()
      } else {
        QVariant::default()
      }
    }
    else {
      QVariant::default()
    }
  }
  
  fn role_names(&self) -> HashMap<i32, QByteArray> {
    let mut map = HashMap::new();
    map.insert(USER_ROLE, "item_type".into());
    map.insert(USER_ROLE + 1, "content".into());
    map
  }
}

#[derive(QObject, Default)]
pub struct ChatModel {
  base: qt_base_class!(trait QObject),
  messages: qt_property!(QVariantList; NOTIFY messages_changed),
  messages_changed: qt_signal!(),

  send_message: qt_method!(fn(msg: String)),
}

impl ChatModel {
  fn send_message(&mut self, msg: String) {
    let message: ChatMessage = msg.into();
  }
}

fn parse_markdown_impl(msg: String) -> Vec<MessageItem> {
  let parser = Parser::new(&msg);
  let mut items = vec![];
  let mut current_text = String::new();
  let mut current_type = MessageItemType::Text;
  let mut in_code_block = false;

  for event in parser {
    match event {
      Event::Start(tag) => {
        current_type = match tag {
          Tag::Heading{ level, id, classes, attrs } => match level {
            HeadingLevel::H1 => MessageItemType::Header1,
            HeadingLevel::H2 => MessageItemType::Header2,
            _ => MessageItemType::Header3,
          },
          Tag::CodeBlock(_) => {
            in_code_block = true;
            MessageItemType::Code
          }
          _ => MessageItemType::Text,
        };
      }
      Event::Text(text) => {
        if in_code_block {
          current_text.push_str(&text);
          current_text.push('\n');
        } else {
          current_text.push_str(&text);
        }
      }
      Event::Code(code) => {
        current_text.push_str(&code);
      }
      Event::End(tag) => match tag {
        _ => {
          // if !current_text.is_empty() {
          //   add_message_item(&mut items, current_type, std::mem::take(&mut current_text));
          // }
          current_type = MessageItemType::Text;
        }
      },
      Event::SoftBreak => {
        current_text.push(' ');
      }
      Event::HardBreak => {
        current_text.push('\n');
      }
      _ => {}
    }
  }

  items
}

pub fn init() {
  qmetaobject::qml_register_type::<ChatModel>(
    cstr!("Chat"), 1, 0, cstr!("ChatModel")
  );

  qmetaobject::qml_register_type::<ChatMessage>(
    cstr!("Chat"), 1, 0, cstr!("ChatMessage")
  );
}