pub trait Encryptor: Send + Sync {
  fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>>;
  fn decrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>>;
}

pub struct NoOpEncryptor;

impl Encryptor for NoOpEncryptor {
  fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
    Ok(data.to_vec())
  }

  fn decrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
    Ok(data.to_vec())
  }
}
