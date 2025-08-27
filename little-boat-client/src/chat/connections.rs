use crate::chat::Message;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Connection(mpsc::Sender<Message>);

impl Connection {
  pub fn send(&mut self, message: Message) -> anyhow::Result<()>{
    self.0.try_send(message)?;
    Ok(())
  }
}
