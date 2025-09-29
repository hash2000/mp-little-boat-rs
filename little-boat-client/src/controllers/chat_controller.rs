mod chat_message;
mod chat_messages_list;

use cstr::cstr;
use crate::controllers::chat_controller::chat_message::ChatMessage;
use crate::controllers::chat_controller::chat_messages_list::ChatMessagesListModel;


pub fn init() {
  qmetaobject::qml_register_type::<ChatMessagesListModel>(
    cstr!("Chat"), 1, 0, cstr!("ChatMessagesListModel")
  );

  qmetaobject::qml_register_type::<ChatMessage>(
    cstr!("Chat"), 1, 0, cstr!("ChatMessage")
  );
}