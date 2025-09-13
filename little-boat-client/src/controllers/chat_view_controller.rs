use std::rc::Rc;
use std::sync::Arc;

use slint::{ComponentHandle, SharedString};

use crate::ui::ApplicationWindow;
use crate::ui::{ChatMessage, ChatMessageFormat, ChatViewAdapter};

pub struct Controller {
  messages: Rc<slint::VecModel<ChatMessage>>,
}

impl Controller {
  pub fn new() -> Self {
    Self { messages: Rc::new(slint::VecModel::from(vec![])) }
  }
}

pub fn init(app: &ApplicationWindow, controller: Arc<Controller>) {
    let app_weak = app.as_weak();
    // app_weak.upgrade_in_event_loop(move |window| {

    // });

    app.global::<ChatViewAdapter>().on_message(move |msg| {
      let bindapp = app_weak.unwrap();
      add_message_into_list(&bindapp, &controller, msg);
    });
}

fn add_message_into_list(app: &ApplicationWindow, controller: &Controller, msg: SharedString) {
  let mut message = ChatMessage::default();
  message.content = SharedString::from(msg);
  message.id = SharedString::from("testuid");
  message.sender = SharedString::from("senderuid");
  message.format = ChatMessageFormat::Markdown;

  controller.messages.push(message);

  app.global::<ChatViewAdapter>().set_messages(controller.messages.clone().into());
}
