use little_boat_core::{run_client_app};

slint::include_modules!();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let ui = ApplicationWindow::new()?;

  ui.run()?;
  Ok(())
}
