// use little_boat_core::run_client_app;

// pub mod ui {
  
// }

// use std::sync::Arc;

// pub mod controllers {
//   pub mod chat_view_controller;
// }

// use controllers::*;


use cstr::cstr;

use qmetaobject::prelude::*;

mod implementations;

qrc!(my_resource,
    "todos/qml" {
        "ui/main.qml",
    },
);

pub async fn run_app() -> anyhow::Result<()> {
    my_resource();
    qml_register_type::<implementations::Todos>(cstr!("RustCode"), 1, 0, cstr!("Todos"));
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/todos/qml/ui/main.qml".into());
    engine.exec();

  
  // tokio::spawn(async move {
  //   if let Err(e) = run_client_app().await {
  //     eprintln!("Client app error: {}", e);
  //   }
  // });

  // let app = ui::ApplicationWindow::new()?;
  // let chat_view = Arc::new(chat_view_controller::Controller::new());
  // chat_view_controller::init(&app, chat_view.clone());

  // app.run()?;
  Ok(())
}