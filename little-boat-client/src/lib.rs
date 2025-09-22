mod frontend;

pub mod controllers {
  pub mod chat_controller;
  pub mod host_controller;
}

use controllers::*;
use qmetaobject::prelude::*;

pub fn run_app() -> anyhow::Result<()> {
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
  qmetaobject::log::init_qt_to_rust();

  frontend::resouces::init();  
  chat_controller::init();
  host_controller::init();

  let mut engine = QmlEngine::new();
  //engine.load_file("qrc:LittleBoat/ui/Styles.qml".into());
  //engine.load_file("qrc:qml/ui/qmldir".into());
  //engine.add_import_path("qrc:/Styles".into());
  ::log::debug!("Load templates");
  engine.load_file("little-boat-client/ui/Main.qml".into());
  ::log::debug!("Load templates comlete");

  // tokio::spawn(async move {
  //   if let Err(e) = run_client_app().await {
  //     eprintln!("Client app error: {}", e);
  //   }
  // });

  let arc_engine = std::sync::Arc::new(engine);

  frontend::hot_reload::watch(
    std::path::PathBuf::from("little-boat-client/ui/"),
    arc_engine.clone());

  arc_engine.exec();

  Ok(())
}
