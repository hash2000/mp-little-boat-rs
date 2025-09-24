use qmetaobject::*;
use std::collections::HashMap;

use crate::controllers::chat_controller::chat_message::ChatMessage;

const CHAT_MESSAGE_LIST_MESSAGES: i32 = USER_ROLE;

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
    let row = self.messages.len();

    // Уведомляем модель о добавлении новой строки
    (self as &mut dyn QAbstractListModel).begin_insert_rows(row as i32, row as i32);
    self.messages.push(message);
    (self as &mut dyn QAbstractListModel).end_insert_rows();

    self.count_changed();
  }
}

#[allow(non_snake_case)]
impl QAbstractListModel for ChatMessagesListModel {
  fn row_count(&self) -> i32 {
    self.messages.len() as i32
  }

  fn data(&self, index: QModelIndex, role: i32) -> QVariant {
    let idx = index.row() as usize;
    if idx >= self.messages.len() {
      return QVariant::default();
    }

    match role {
      CHAT_MESSAGE_LIST_MESSAGES => self.messages[idx].clone().into(),
      _ => QVariant::default(),
    }
  }

  fn role_names(&self) -> HashMap<i32, QByteArray> {
    let mut map: HashMap<i32, QByteArray> = HashMap::new();
    map.insert(CHAT_MESSAGE_LIST_MESSAGES, "messageData".into());
    map
  }
}
