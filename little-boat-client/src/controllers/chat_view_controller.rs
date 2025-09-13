use std::rc::Rc;

use slint::{ComponentHandle, Model, SharedString};

use crate::ui::ApplicationWindow;
use crate::ui::{ChatViewAdapter, ChatMessage, ChatMessageFormat};


pub struct ChatViewController {
  messages: Rc<slint::VecModel<ChatMessage>>,
}

pub fn init(app: &ApplicationWindow) {

  let app_weak = app.as_weak();
  // app_weak.upgrade_in_event_loop(move |window| {
  //   let adapter = window.global::<ChatViewAdapter>();
  //   let messages : Vec<ChatMessage> = adapter.get_messages().iter().collect();

  //   let mut message = Rc::new(ChatMessage::default());
  //   message.content = SharedString::from(msg);
  //   message.id = SharedString::from("testuid");
  //   message.sender = SharedString::from("senderuid");
  //   message.format = ChatMessageFormat::Markdown;

  //   messages.push(message);

  // });


  app.global::<ChatViewAdapter>().on_message(move|msg| {
    let mut message = ChatMessage::default();
    message.content = SharedString::from(msg);
    message.id = SharedString::from("testuid");
    message.sender = SharedString::from("senderuid");
    message.format = ChatMessageFormat::Markdown;

    let binding = app_weak.unwrap();
    let adapter = binding.global::<ChatViewAdapter>();

    let mut messages: Vec<ChatMessage> = adapter.get_messages().iter().collect();
    messages.push(message);

    let controller = ChatViewController {
      messages: Rc::new(slint::VecModel::from(messages))
    };

    adapter.set_messages(controller.messages.clone().into());
  });




}
