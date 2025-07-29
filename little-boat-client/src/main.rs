
slint::include_modules!();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let ui = ApplicationWindow::new()?;

  ui.run()?;
  Ok(())
}
