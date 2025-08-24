mod application;
use little_boat_core::run_client_app;
use crate::application::TuiApplication;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tokio::spawn(async move {
    if let Err(e) = run_client_app().await {
      eprintln!("Client app error: {}", e);
    }
  });

  let mut app = TuiApplication::new()?;
  app.run(ratatui::init()).await?;

  Ok(())
}
