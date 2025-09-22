use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use qmetaobject::*;
use std::{collections::HashMap, ops::Index};

use crate::controllers::chat_controller::chat_message::ChatMessage;

const CHAT_MESSAGE_LIST_MESSAGES: i32 = USER_ROLE;



#[derive(QObject, Default)]
pub struct ChatMessagesListModel {
  base: qt_base_class!(trait QAbstractTableModel),
  count: qt_property!(i32; READ row_count NOTIFY count_changed),
  count_changed: qt_signal!(),
  messages: Vec<ChatMessage>,

  send_message: qt_method!(fn(msg: String)),
}

impl ChatMessagesListModel {
  fn send_message(&mut self, msg: String) {
    let message: ChatMessage = msg.into();
    self.messages.push(message);
  }
}

#[allow(non_snake_case)]
impl QAbstractTableModel for ChatMessagesListModel {
  fn row_count(&self) -> i32 {
    self.messages.len() as i32
  }
  
  fn column_count(&self) -> i32 {
    2
  }  

  fn data(&self, index: QModelIndex, role: i32) -> QVariant {
    let idx = index.row() as usize;
    if idx >= self.messages.len() {
      QString::from(format!("Unknown role: {}", role)).to_qvariant()
    } else {
      let row = self.messages.get(index.row() as usize).unwrap();

      match index.column() {
        0 => self.messages[idx].clone().into(),
        _ => QString::from("").to_qvariant(),
      }
    }
  }
  
  fn role_names(&self) -> HashMap<i32, QByteArray> {
    let mut map = HashMap::new();
    map.insert(CHAT_MESSAGE_LIST_MESSAGES, "messages".into());
    map
  }

}