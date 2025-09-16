use std::rc::Rc;
use std::sync::Arc;
use pulldown_cmark::{html, Event, HeadingLevel, Options, Parser, Tag};

use slint::{ComponentHandle, SharedString};

use crate::ui::{
  ApplicationWindow, 
  ChatMessageContentItem, 
  ChatMessage, 
  ChatMessageFormat,
  ChatViewAdapter,
  ChatMessageContentType
};

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
  message.content_raw = msg.clone();
  message.content = parse_raw_text_to_content_items(&msg).clone().into();
  message.id = SharedString::from("testuid");
  message.sender = SharedString::from("senderuid");
  message.format = ChatMessageFormat::Markdown;

  controller.messages.push(message);

  app.global::<ChatViewAdapter>().set_messages(controller.messages.clone().into());
}

fn parse_raw_text_to_content_items(msg: &str) -> Rc<slint::VecModel<ChatMessageContentItem>> {
  let parser = Parser::new(msg);
  let items: Rc<slint::VecModel<ChatMessageContentItem>> = Rc::new(slint::VecModel::from(vec![]));
  
  for (event, range) in parser.into_offset_iter() {
    let text = &msg[range];
    let mut content_text = SharedString::new();
    content_text.push_str(text);

    match event {
      Event::Start(tag) => {
        match tag {
          Tag::Heading { level, id, classes, attrs } => {
            let content_item = ChatMessageContentItem {
              data: content_text,
              content_type: ChatMessageContentType::Citation,
              font_size_mul: get_heading_font_size_mul(level)
            };
            items.push(content_item);
          },
          // Tag::Paragraph => {

          // }
          // Tag::Link { link_type, dest_url, title, id } => {

          // }
          // Tag::Emphasis => {

          // }
          // Tag::Strong => {

          // }
          // Tag::CodeBlock(code_block) => {

          // }
          // Tag::HtmlBlock => {

          // }
          _ => {
            let content_item = ChatMessageContentItem {
              data: content_text,
              content_type: ChatMessageContentType::Text,
              font_size_mul: 0.0,
            };
            items.push(content_item);
          }
        }
      }
      _ => {
      }
    };    
  }

  items
}

fn get_heading_font_size_mul(level: HeadingLevel) -> f32 {
  match level {
    HeadingLevel::H1 => 1.1,
    HeadingLevel::H2 => 1.2,
    HeadingLevel::H3 => 1.3,
    HeadingLevel::H4 => 1.4,
    HeadingLevel::H5 => 1.5,
    HeadingLevel::H6 => 1.6,
  }
}