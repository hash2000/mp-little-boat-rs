use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use qmetaobject::*;
use std::{collections::HashMap, ops::Index};

use crate::controllers::chat_controller::chat_message::ChatMessage;

const CHAT_MESSAGE_LIST_MESSAGE: i32 = USER_ROLE;



#[derive(QObject, Default)]
pub struct ChatMessagesListModel {
  base: qt_base_class!(trait QAbstractListModel),
  count: qt_property!(i32; READ row_count NOTIFY count_changed),
  count_changed: qt_signal!(),
  messages: Vec<ChatMessage>,

  send_message: qt_method!(fn(msg: String)),
}

impl ChatMessagesListModel {
  fn send_message(&mut self, msg: String) {
    let message: ChatMessage = msg.into();

  }
}


impl QAbstractListModel for ChatMessagesListModel {
  fn row_count(&self) -> i32 {
    self.messages.len() as i32
  }
  
  fn data(&self, index: QModelIndex, role: i32) -> QVariant {
    let idx = index.row() as usize;
    if idx >= self.messages.len() {
      QVariant::default()
    } else {
      match role {
        CHAT_MESSAGE_LIST_MESSAGE => QString::from("test message").into(), // self.messages[idx].clone(),
        _ => QVariant::default()
      }
    }
  }
  
  fn role_names(&self) -> HashMap<i32, QByteArray> {
    let mut map = HashMap::new();
    map.insert(CHAT_MESSAGE_LIST_MESSAGE, "message".into());
    map
  }
}