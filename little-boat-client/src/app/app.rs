use std::collections::HashMap;
use std::time::Duration;

use iced::widget::{button, column, container, row, text, tooltip};
use iced::window::events;
use iced::{Alignment, Element, Length, Subscription, Task, Theme, window};
use iced_widget::{scrollable, Space};

use crate::app::{Dashboard, DashboardId, Message};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarAction {
  Settings,
  Minimize,
  Close,
}

pub struct App {
  window_id: window::Id,
  active_dashboard: Option<DashboardId>,
  dashboards: HashMap<DashboardId, Dashboard>,
  dashboard_order: Vec<DashboardId>,

  sidebar_width: f32,
  is_resizing: bool,
  is_minimized: bool,
  show_settings: bool,
}

impl App {
  pub fn new() -> (Self, Task<Message>) {
    let (id, open) = window::open(window::Settings::default());

    (
      Self {
        window_id: id,
        active_dashboard: None,
        dashboards: HashMap::new(),
        dashboard_order: Vec::new(),
        sidebar_width: 250.0,
        is_resizing: false,
        is_minimized: false,
        show_settings: false,
      },
      open.map(Message::WindowOpened),
    )
  }

  pub fn title(&self, _window_id: window::Id) -> String {
    String::from("Little Boat")
  }

  pub fn theme(&self, _window_id: window::Id) -> Theme {
    Theme::KanagawaDragon
  }

  pub fn scale_factor(&self, _window_id: window::Id) -> f32 {
    1.0
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::WindowOpened(id) => Task::none(),
      Message::Window(id, event) => {
        if id == self.window_id {
          self.process_window_events(event)
        } else {
          Task::none()
        }
      }
      Message::Tick(now) => Task::none(),
      Message::Event(id, event) => Task::none(),
      Message::ToggleMinimize => Task::none(),
      Message::ToggleSettings => Task::none(),
    }
  }

  pub fn view(&self, id: window::Id) -> Element<'_, Message> {
    if id != self.window_id { 
      column![].into() 
    } else { 
      column![].into() 
    }
  }

  fn view_sidebar(&self) -> Element<Message> {
    let action_buttons = self.view_action_buttons();
    let dashboard_buttons = self.view_dashboard_buttons();
    
    column![
      // Верхняя панель с действиями
      container(action_buttons)
        .padding(10)
        .width(Length::Fill),
      
      horizontal_rule(1),
      
      // Прокручиваемая область с кнопками дашбордов
      scrollable(
        container(dashboard_buttons)
          .padding(10)
          .width(Length::Fill)
      )
      .height(Length::Fill),
    ]
    .height(Length::Fill)
    .style(container::rounded_box)
    .into()
  }

  fn view_action_button<'a>(
    &'a self,
    action: SidebarAction,
    tooltip_text: &'a str,
  ) -> Element<'a, Message> {
    let (icon, message) = match action {
      SidebarAction::Settings => ("⚙", Message::ToggleSettings),
      SidebarAction::Minimize => ("─", Message::ToggleMinimize),
      SidebarAction::Close => ("✕", Message::ToggleMinimize), // Пример
    };

    let button = button(text(icon).size(14))
      .padding(5)
      .style(button::secondary);

    tooltip(button, tooltip_text, tooltip::Position::Bottom)
      .into()
  }

  fn view_action_buttons<'a>(&'a self) -> Element<'a, Message> {
    // Кнопки выровнены справа налево
    row![
      Space::new().width(Length::Fill),
      self.view_action_button(SidebarAction::Close, "Закрыть"),
      self.view_action_button(SidebarAction::Minimize, "Свернуть"),
      self.view_action_button(SidebarAction::Settings, "Настройки"),
    ]
    .spacing(10)
    .align_y(Alignment::End)
    .into()
  }

  fn view_minimized_sidebar<'a>(&'a self) -> Element<'a, Message> {
    let action_buttons = row![
      self.view_action_button(SidebarAction::Minimize, "Развернуть"),
      self.view_action_button(SidebarAction::Settings, "Настройки"),
    ]
    .spacing(5);

    container(action_buttons).width(60).height(Length::Fill).style(container::rounded_box).into()
  }

    fn view_dashboard_button(&self, dashboard: &Dashboard) -> Element<Message> {
      let is_active = self.active_dashboard.as_ref() == Some(dashboard.id());
      
      let button_style = if is_active {
        button::primary
      } else {
        button::secondary
      };
      
      let close_button = button(text("×").size(12))
        .padding(2)
        .on_press(Message::DashboardClosed(dashboard.id().clone()))
        .style(button::destructive);
      
      row![
        button(
          text(dashboard.title())
            .width(Length::Fill)
        )
        .style(button_style)
        .width(Length::Fill)
        .on_press(Message::DashboardSelected(dashboard.id().clone())),
        close_button,
      ]
      .spacing(5)
      .align_items(Alignment::Center)
      .into()
  }

  pub fn subscription(&self) -> Subscription<Message> {
    let tick = iced::time::every(Duration::from_secs(1)).map(Message::Tick);

    let subscriptions = vec![
      events().map(|(window, event)| Message::Event(window, event)),
      window::events().map(|(window, event)| Message::Window(window, event)),
      tick,
    ];

    Subscription::batch(subscriptions)
  }
}
