use little_boat_core::{run_client_app};

#[cfg(feature = "slint-ui")]
slint::include_modules!();

fn main() -> anyhow::Result<()> {
  #[cfg(feature = "slint-ui")]
  return Ok(main_full_featured()?);

  #[cfg(feature = "console-ui")]
  return Ok(main_poc_featured()?);

  #[cfg(not(any(feature = "slint-ui", feature = "console-ui")))]
  Ok(())
}

#[cfg(feature = "slint-ui")]
#[tokio::main]
async fn main_full_featured() -> anyhow::Result<()> {
  let ui = ApplicationWindow::new()?;

  ui.run()?;
  Ok(())
}

#[cfg(feature = "console-ui")]
#[tokio::main]
async fn main_poc_featured() -> anyhow::Result<()> {
  run_client_app().await
}
