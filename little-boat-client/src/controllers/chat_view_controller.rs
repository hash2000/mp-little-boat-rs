use slint::ComponentHandle;

use crate::ui::ApplicationWindow;


pub fn init(app: &ApplicationWindow) {
  let weak = app.as_weak();
  weak.unwrap().global::<ChatMessageListView>().on_message(|msg| {

  });
}
