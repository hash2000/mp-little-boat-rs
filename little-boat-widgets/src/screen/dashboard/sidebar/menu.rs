#[derive(Debug, Clone, Copy)]
pub enum Menu {
  Version,
}

impl Menu {
  pub fn list() -> Vec<Self> {
    vec![
      Menu::Version,
    ]
  }
}
