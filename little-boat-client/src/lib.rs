// use little_boat_core::run_client_app;

// pub mod ui {
  
// }

// use std::sync::Arc;

pub mod controllers {
  pub mod chat_controller;
}

use controllers::*;
use qmetaobject::prelude::*;


qrc!(ui_resources,
  "qml" {
    "ui/chat/main.qml",
  },
);

pub async fn run_app() -> anyhow::Result<()> {

  chat_controller::init();

  ui_resources();

  let mut engine = QmlEngine::new();
  engine.load_file("qrc:/qml/ui/chat/main.qml".into());
    

  
  // tokio::spawn(async move {
  //   if let Err(e) = run_client_app().await {
  //     eprintln!("Client app error: {}", e);
  //   }
  // });

  engine.exec();
  Ok(())
}