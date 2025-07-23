use crate::database::encryption::Encryptor;

use chacha20poly1305::{
  ChaCha20Poly1305,
  Key,   // 32-байтный ключ
  Nonce, // 12-байтный nonce
  aead::{Aead, KeyInit, generic_array::GenericArray},
};

use generic_array::typenum::U12;
use rand::RngCore;

pub struct ChaChaEncryptor {
  cipher: ChaCha20Poly1305,
}

impl ChaChaEncryptor {
  pub fn new(key: &[u8]) -> anyhow::Result<Self> {
    if key.len() != 32 {
      return Err(anyhow::anyhow!("Key must be 32 bytes (256 bits)"));
    }

    let key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    Ok(Self { cipher })
  }

  fn generate_nonce() -> GenericArray<u8, U12> {
    let mut nonce = GenericArray::default();
    rand::rng().fill_bytes(&mut nonce);
    nonce
  }
}

impl Encryptor for ChaChaEncryptor {
  fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let nonce = Self::generate_nonce();
    let mut ciphertext = self
      .cipher
      .encrypt(&nonce, data.as_ref())
      .map_err(|e| anyhow::anyhow!("ChaCha20Poly1305 encryption failed: {}", e))?;

    // Добавляем nonce в начало
    let mut result = nonce.to_vec();
    result.append(&mut ciphertext);
    Ok(result)
  }

  fn decrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
    if data.len() < 12 {
      return Err(anyhow::anyhow!("Data too short to contain nonce"));
    }

    let nonce = Nonce::from_slice(&data[..12]);
    self
      .cipher
      .decrypt(nonce, &data[12..])
      .map_err(|e| anyhow::anyhow!("ChaCha20Poly1305 decryption failed: {}", e))
  }
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_aes_encrypt_decrypt() -> anyhow::Result<()> {
    let key = [0u8; 32]; // Тестовый ключ
    let encryptor = ChaChaEncryptor::new(&key)?;

    let plaintext = b"Hello, encrypted world!";
    let encrypted = encryptor.encrypt(plaintext)?;
    let decrypted = encryptor.decrypt(&encrypted)?;

    assert_eq!(plaintext, decrypted.as_slice());
    Ok(())
  }
}
