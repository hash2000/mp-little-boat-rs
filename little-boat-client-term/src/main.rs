use little_boat_core::run_client_app;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tokio::spawn(async move {
    if let Err(e) = run_client_app().await {
      eprintln!("Client app error: {}", e);
    }
  });

  Ok(())
}
