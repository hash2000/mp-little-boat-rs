use little_boat_core::run_client_app;

pub mod ui {
  slint::include_modules!();
}

use std::sync::Arc;

use slint::*;

pub mod controllers {
  pub mod chat_view_controller;
}

use controllers::*;

pub async fn run_app() -> anyhow::Result<()> {
  // tokio::spawn(async move {
  //   if let Err(e) = run_client_app().await {
  //     eprintln!("Client app error: {}", e);
  //   }
  // });

  let app = ui::ApplicationWindow::new()?;
  let chat_view = Arc::new(chat_view_controller::Controller::new());
  chat_view_controller::init(&app, chat_view.clone());

  app.run()?;
  Ok(())
}