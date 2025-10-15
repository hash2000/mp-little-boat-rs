mod menu;

use menu::Menu;

use iced::{
  Length, Task,
  widget::{button, pane_grid, row, stack, text},
  window,
};
use std::time::Duration;

use crate::{
  Theme,
  appearance::theme,
  widgets::{
    Element, Text,
    context_menu::{self, context_menu},
  },
};
use crate::{
  font,
  widgets::widgets_base::{self, buffer, environment},
};

const CONFIG_RELOAD_DELAY: Duration = Duration::from_secs(1);

#[derive(Debug, Clone)]
pub enum Message {
  New(buffer::Upstream),
  Popout(buffer::Upstream),
  Focus(window::Id, pane_grid::Pane),
  Replace(buffer::Upstream),
  Close(window::Id, pane_grid::Pane),
  Swap(window::Id, pane_grid::Pane),
  Detach(buffer::Upstream),
  Leave(buffer::Upstream),
  ToggleInternalBuffer(buffer::Internal),
  ToggleCommandBar,
  ToggleThemeEditor,
  //  ReloadConfigFile,
  //  ConfigReloaded(Result<Config, config::Error>),
  OpenReleaseWebsite,
  OpenDocumentation,
  OpenConfigFile,
  ReloadComplete,
  MarkAsRead(buffer::Upstream),
  //  MarkServerAsRead(Server),
}

#[derive(Debug, Clone)]
pub enum Event {
  New(buffer::Upstream),
  Popout(buffer::Upstream),
  Focus(window::Id, pane_grid::Pane),
  Replace(buffer::Upstream),
  Close(window::Id, pane_grid::Pane),
  Swap(window::Id, pane_grid::Pane),
  Detach(buffer::Upstream),
  Leave(buffer::Upstream),
  ToggleInternalBuffer(buffer::Internal),
  ToggleCommandBar,
  ToggleThemeEditor,
  OpenReleaseWebsite,
  OpenDocumentation,
  OpenConfigFile,
  //    ConfigReloaded(Result<Config, config::Error>),
  MarkAsRead(buffer::Upstream),
  //   MarkServerAsRead(Server),
}

#[derive(Clone)]
pub struct Sidebar {
  pub hidden: bool,
  reloading_config: bool,
}

impl Default for Sidebar {
  fn default() -> Self {
    Self::new()
  }
}

impl Sidebar {
  pub fn new() -> Self {
    Self { hidden: false, reloading_config: false }
  }

  pub fn toggle_visibility(&mut self) {
    self.hidden = !self.hidden;
  }

  pub fn update(&mut self, message: Message) -> (Task<Message>, Option<Event>) {
    match message {
      Message::New(source) => (Task::none(), Some(Event::New(source))),
      Message::Popout(source) => (Task::none(), Some(Event::Popout(source))),
      Message::Focus(window, pane) => (Task::none(), Some(Event::Focus(window, pane))),
      Message::Replace(source) => (Task::none(), Some(Event::Replace(source))),
      Message::Close(window, pane) => (Task::none(), Some(Event::Close(window, pane))),
      Message::Swap(window, pane) => (Task::none(), Some(Event::Swap(window, pane))),
      Message::Detach(buffer) => (Task::none(), Some(Event::Detach(buffer))),
      Message::Leave(buffer) => (Task::none(), Some(Event::Leave(buffer))),
      Message::ToggleInternalBuffer(buffer) => {
        (Task::none(), Some(Event::ToggleInternalBuffer(buffer)))
      }
      Message::ToggleCommandBar => (Task::none(), Some(Event::ToggleCommandBar)),
      Message::ToggleThemeEditor => (Task::none(), Some(Event::ToggleThemeEditor)),
      // Message::ReloadConfigFile => {
      //     self.reloading_config = true;
      //     (Task::perform(Config::load(), Message::ConfigReloaded), None)
      // }
      // Message::ConfigReloaded(config) => (
      //     Task::perform(time::sleep(CONFIG_RELOAD_DELAY), |()| {
      //         Message::ReloadComplete
      //     }),
      //     Some(Event::ConfigReloaded(config)),
      // ),
      Message::OpenReleaseWebsite => (Task::none(), Some(Event::OpenReleaseWebsite)),
      Message::ReloadComplete => {
        self.reloading_config = false;
        (Task::none(), None)
      }
      Message::OpenDocumentation => (Task::none(), Some(Event::OpenDocumentation)),
      Message::MarkAsRead(buffer) => (Task::none(), Some(Event::MarkAsRead(buffer))),
      // Message::MarkServerAsRead(server) => {
      //     (Task::none(), Some(Event::MarkServerAsRead(server)))
      // }
      Message::OpenConfigFile => (Task::none(), Some(Event::OpenConfigFile)),
    }
  }

  fn user_menu_button<'a>(
    &self,
    keyboard: &'a widgets_base::config::Keyboard,
    theme: &'a Theme,
  ) -> Element<'a, Message> {
    let base = button(text("_")).padding(5).width(Length::Shrink);
    let menu = Menu::list();

    if menu.is_empty() {
      base.into()
    } else {
      stack![
        context_menu(
          context_menu::MouseButton::Left,
          context_menu::Anchor::Widget,
          context_menu::ToggleBehavior::Close,
          base,
          menu,
          move |menu, length| {
            let context_button = |title: Text<'a>,
                                  keybind: Option<&widgets_base::shortcut::KeyBind>,
                                  icon: Text<'a>,
                                  message: Message| {
              button(
                row![
                  icon.width(Length::Fixed(12.0)),
                  title,
                  keybind.map(|kb| {
                    text(format!("({kb})"))
                      .shaping(text::Shaping::Advanced)
                      .size(theme::TEXT_SIZE - 2.0)
                      .style(theme::text::secondary)
                    // .font_maybe(
                    //     theme::font_style::secondary(
                    //         theme,
                    //     )
                    //     .map(font::get),
                    // )
                  }),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
              )
              .width(length)
              .padding(5)
              .on_press(message)
              .into()
            };
            match menu {
              Menu::Version => context_button(
                text(format!("{} ({})", environment::APP_NAME, environment::APP_VERSION))
                  .style(theme::text::secondary)
                  .font_maybe(theme::font_style::secondary(theme)
                  .map(font::get)),
                  None,
                  text("^"),
                  Message::OpenReleaseWebsite
              ),
            }
          },
        ),
        // if show_notification_dot {
        //     Some(
        //         container(
        //             icon::dot().style(theme::text::tertiary).size(8),
        //         )
        //         .padding(padding::left(13).top(2)),
        //     )
        // } else {
        //     None
        // },
      ]
      .into()
    }
  }
}
