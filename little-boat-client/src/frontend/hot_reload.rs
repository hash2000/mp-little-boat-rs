use qmetaobject;
use std::sync::Arc;
use std::{path::PathBuf, thread, time::Instant};

#[cfg(debug_assertions)]
pub fn watch(path: PathBuf, engine: Arc<qmetaobject::QmlEngine>) {
  use notify::{self, Watcher};

  thread::spawn(move || {
    let (notify_sender, notify_receiver) = std::sync::mpsc::channel();
    let mut watcher =
      notify::RecommendedWatcher::new(notify_sender, notify::Config::default()).unwrap();

    if let Err(error) = watcher.watch(&path, notify::RecursiveMode::Recursive) {
      log::error!("Hot reload failed to initialize: {}", error);
      return;
    }

    log::info!("Hot reload is in use");

    let mut last_reload = Instant::now();
    let qml_path = path.clone();
    let engine_clone = engine.clone();

    loop {
      let event_result = notify_receiver.recv().unwrap();
      if let Ok(event) = event_result {
        use std::sync::Mutex;

        use qmetaobject::{QString, QmlEngine};

        if event.kind.is_access()
          || last_reload.elapsed().as_millis() < 500
          || event.paths.iter().any(|p| p.to_string_lossy().contains("~"))
        {
          continue;
        }

        // let callback = qmetaobject::queued_callback(|engine_mut: Arc<Mutex<QmlEngine>>| {
        //   log::info!("Reloading QML...");

        //   if let Ok(engine_guard) = engine_mut.lock() {
        //     engine_guard.trim_component_cache();
        //     engine_guard.clear_component_cache();
        //     last_reload = Instant::now();
        //   }
          
        //   log::info!("QML reloaded successfully");
        // });

        // callback(Arc::new(Mutex::new(engine.clone())));

        log::info!("Reload triggered by file {}", event.paths[0].display());
        engine.trim_component_cache();
        engine.clear_component_cache();
        //engine.load_file(QString::from(qml_path.to_string_lossy().to_string()));
        last_reload = Instant::now();
      }
    }
  });
}

#[cfg(not(debug_assertions))]
pub fn watch(_path: PathBuf, _engine: Arc<qmetaobject::QmlEngine>) {
  // No hot reload in release mode.
}
