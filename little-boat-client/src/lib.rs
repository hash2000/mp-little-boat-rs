
pub mod ui {
  slint::include_modules!();
}

use slint::*;

pub mod controllers {
  pub mod chat_view_controller;
}

use controllers::*;

pub async fn run_app() -> anyhow::Result<()> {
  let app = ui::ApplicationWindow::new()?;
  chat_view_controller::init(&app);

  app.run()?;
  Ok(())
}