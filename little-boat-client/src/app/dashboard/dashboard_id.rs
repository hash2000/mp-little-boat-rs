use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DashboardId(Uuid);

impl DashboardId {
  pub fn new() -> Self {
    Self (
      Uuid::new_v4()
    )
  }
}

impl Default for DashboardId {
  fn default() -> Self {
    Self::new()
  }
}