use little_boat_client::run_app;


fn main() -> anyhow::Result<()> {

  let config = env_logger::Env::default()
    .default_filter_or("info");

  env_logger::Builder::from_env(config)
    .init();

  run_app()?;
  
  Ok(())
}
