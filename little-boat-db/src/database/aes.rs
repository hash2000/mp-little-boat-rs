use crate::database::encryption::Encryptor;

use aes_gcm::{
  Aes256Gcm,
  aead::consts::U12,
  aead::{Aead, KeyInit, generic_array::GenericArray},
};
use rand::RngCore;

pub struct AesEncryptor {
  cipher: Aes256Gcm,
}

impl AesEncryptor {
  pub fn new(key: &[u8]) -> anyhow::Result<Self> {
    if key.len() != 32 {
      return Err(anyhow::anyhow!("Key must be 32 bytes (256 bits)"));
    }

    let key = GenericArray::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    Ok(Self { cipher })
  }

  fn generate_nonce() -> GenericArray<u8, U12> {
    let mut nonce = GenericArray::default();
    rand::rng().fill_bytes(&mut nonce);
    nonce
  }
}

impl Encryptor for AesEncryptor {
  fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let nonce = Self::generate_nonce();
    let mut ciphertext = self
      .cipher
      .encrypt(&nonce, data.as_ref())
      .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    // Добавляем nonce в начало
    let mut result = nonce.to_vec();
    result.append(&mut ciphertext);
    Ok(result)
  }

  fn decrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
    if data.len() < 12 {
      return Err(anyhow::anyhow!("Data too short to contain nonce"));
    }

    let nonce = GenericArray::from_slice(&data[..12]);
    self.cipher.decrypt(nonce, &data[12..]).map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_aes_encrypt_decrypt() -> anyhow::Result<()> {
    let key = [0u8; 32]; // Тестовый ключ
    let encryptor = AesEncryptor::new(&key)?;

    let plaintext = b"Hello, encrypted world!";
    let encrypted = encryptor.encrypt(plaintext)?;
    let decrypted = encryptor.decrypt(&encrypted)?;

    assert_eq!(plaintext, decrypted.as_slice());
    Ok(())
  }
}
