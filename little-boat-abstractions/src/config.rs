
pub trait IConfigReader : Send + Sync {

  fn has_flag(&self, key: &[u8], def: bool) -> bool;

  fn get_str(&self, key: &[u8], def: &str) -> String;

  fn get_json(&self, key: &[u8]) -> Option<simd_json::OwnedValue>;

  fn get_bool(&self, key: &[u8], def: bool) -> bool;

  fn get_float(&self, key: &[u8], def: f64) -> f64;

  fn get_int(&self, key: &[u8], def: usize) -> usize;
}

pub trait IConfigWriter : Send + Sync {

  fn set_flag(&self, key: &[u8]);

  fn set_str(&self, key: &[u8], value: &str) -> anyhow::Result<()>;

  fn set_json(&self, key: &[u8], value: simd_json::OwnedValue) -> anyhow::Result<()>;

  fn set_bool(&self, key: &[u8], value: bool) -> anyhow::Result<()>;

  fn set_float(&self, key: &[u8], value: f64) -> anyhow::Result<()>;

  fn set_int(&self, key: &[u8], value: usize) -> anyhow::Result<()>;
}

pub trait IConfig: IConfigReader + IConfigWriter + Send + Sync {}