mod client;
mod config;
mod services;

pub use crate::client::ClientApp;

pub async fn run_client_app() -> anyhow::Result<()> {
    let mut app = ClientApp::new()?;
    app.run().await
}