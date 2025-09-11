
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  little_boat_client::run_app().await?;
  Ok(())
}
