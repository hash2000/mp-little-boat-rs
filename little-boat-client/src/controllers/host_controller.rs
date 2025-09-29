use qmetaobject::*;

#[derive(QObject, Default)]
#[allow(non_snake_case)]
struct HostController {
  base: qt_base_class!(trait QAbstractTableModel),
}

impl QAbstractTableModel for HostController {
  fn row_count(&self) -> i32 {
    todo!()
  }

  fn column_count(&self) -> i32 {
    todo!()
  }

  fn data(&self, index: qmetaobject::QModelIndex, role: i32) -> qmetaobject::QVariant {
    todo!()
  }
}

pub fn init() {

  
}
