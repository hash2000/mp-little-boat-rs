use anyhow::Result;

#[cfg(feature = "slint-ui")]
slint::include_modules!();

fn main() -> Result<()> {
  #[cfg(feature = "slint-ui")]
  return Ok(main_full_featured()?);

  #[cfg(feature = "console-ui")]
  return Ok(main_poc_featured()?);

  #[cfg(not(any(feature = "slint-ui", feature = "console-ui")))]
  Ok(())
}

#[cfg(feature = "slint-ui")]
#[tokio::main]
async fn main_full_featured() -> Result<()> {
  let ui = ApplicationWindow::new()?;

  ui.run()?;
  Ok(())
}

#[cfg(feature = "console-ui")]
#[tokio::main]
async fn main_poc_featured() -> Result<()> {
  Ok(())
}
