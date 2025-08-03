use little_boat_abstractions::IConfigReader;

// Mock Config
#[derive(Clone)]
pub struct TestConfig;

impl IConfigReader for TestConfig {
  fn has_flag(&self, key: &[u8], def: bool) -> bool {
    match key {
      b"service.signaling.enabled" => true,
      _ => def,
    }
  }

  fn get_str(&self, key: &[u8], def: &str) -> String {
    match key {
      b"service.signaling.host" => "127.0.0.1".to_string(),
      _ => def.to_string(),
    }
  }

  fn get_json(&self, _key: &[u8]) -> Option<simd_json::OwnedValue> {
    None
  }

  fn get_bool(&self, _key: &[u8], def: bool) -> bool {
    def
  }

  fn get_float(&self, _key: &[u8], def: f64) -> f64 {
    def
  }

  fn get_int(&self, key: &[u8], def: usize) -> usize {
    match key {
      b"service.signaling.port" => 0, // 0 = random
      _ => def,
    }
  }
}
