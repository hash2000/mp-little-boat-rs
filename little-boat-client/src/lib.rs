mod app;

use crate::app::App;

pub fn run_app() -> anyhow::Result<()> {

  iced::daemon(App::new, App::update, App::view)
    .subscription(App::subscription)
    .title(App::title)
    .theme(App::theme)
    .scale_factor(App::scale_factor)
    .run()
    .inspect_err(|err| log::error!("{err}"))?;

  Ok(())
}

