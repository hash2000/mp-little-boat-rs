mod application;

use std::io::stdout;

use little_boat_core::run_client_app;
use crate::application::TuiApplication;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tokio::spawn(async move {
    if let Err(e) = run_client_app().await {
      eprintln!("Client app error: {}", e);
    }
  });


  let app = TuiApplication::new(stdout())?;

  Ok(())
}
