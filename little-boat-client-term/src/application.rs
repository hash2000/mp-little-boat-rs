mod event;
mod service_status;
mod state;

use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode, KeyModifiers, 
  poll as crosstermpool,
  read as crosstermread
};
use little_boat_abstractions::{
  ChatEvent, ServiceEvent, ServiceEventMessage, SignalingEvent, SystemEvent,
};
use little_boat_core::{run_client_app, ClientApp};
use tokio::sync::mpsc;

pub use crate::application::event::{AppEvent, poll_event};
pub use crate::application::service_status::ServiceStatus;
pub use crate::application::state::AppState;
use crate::ui::TuiApp;

pub struct Application {
  pub state: AppState,
  pub tui: TuiApp,
  pub client: ClientApp,
  pub services: Vec<ServiceStatus>,
  pub chat_messages: Vec<String>,
  pub connected_users: Vec<String>,
  pub status_message: String,
  pub last_tick: Instant,
}

impl Application {
  pub fn new() -> anyhow::Result<Self> {
    let app = Self {
      state: AppState::Initializing,
      tui: TuiApp::new()?,
      client: ClientApp::new()?,
      services: vec![
        ServiceStatus {
          name: "signaling".to_string(),
          status: "Stopped".to_string(),
          last_update: Instant::now(),
        },
        ServiceStatus {
          name: "chat".to_string(),
          status: "Stopped".to_string(),
          last_update: Instant::now(),
        },
      ],
      chat_messages: Vec::new(),
      connected_users: Vec::new(),
      status_message: "Welcome to Little Boat Chat!".to_string(),
      last_tick: Instant::now(),
    };

    Ok(app)
  }

  pub async fn run(
    &mut self,
    terminal: &mut ratatui::Terminal<impl ratatui::backend::Backend>,
    event_rx: &mut mpsc::UnboundedReceiver<AppEvent>,
  ) -> anyhow::Result<()> {
    let (service_tx, mut service_rx) = mpsc::unbounded_channel::<ServiceEvent>();

    self.client.run().await?;

    tokio::spawn(async move {
      if let Err(e) = run_client_app().await {
        eprintln!("Client app error: {}", e);
      }
    });

    loop {
      if !self.begin_frame(terminal)? {
        break;
      }

      match tokio::time::timeout(Duration::from_millis(100), event_rx.recv()).await {
        Ok(Some(event)) => {
          if self.handle_event(event, &service_tx).await? {
            break;
          }
        }
        Ok(None) => break,
        Err(_) => {
          if self.last_tick.elapsed() >= Duration::from_secs(1) {
            self.last_tick = Instant::now();
            self.handle_event(AppEvent::Tick, &service_tx).await?;
          }
        }
      }

      self.client.process_service_events().await?;
    }


    self.client.wait_services().await?;

    Ok(())
  }

  fn begin_frame(
    &mut self,
    terminal: &mut ratatui::Terminal<impl ratatui::backend::Backend>,
  ) -> anyhow::Result<(bool)> {
    if self.tui.should_quit() {
      return Ok(false);
    }

    self.tui.cleanup_notifications();

    terminal.draw(|frame| {
      self.tui.render(frame);
    })?;

    Ok(true)
  }

  async fn handle_event(
    &mut self,
    event: AppEvent,
    service_tx: &mpsc::UnboundedSender<ServiceEvent>,
  ) -> anyhow::Result<bool> {
    match event {
      AppEvent::Key(key) => {

        if key.kind == crossterm::event::KeyEventKind::Press
          && key.modifiers.contains(KeyModifiers::CONTROL)
        {
          match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
              self.state = AppState::Shutdown;
              return Ok(true);
            }
            KeyCode::Char('s') => {
              self.start_service("signaling", service_tx).await?;
            }
            KeyCode::Char('a') => {
              self.start_service("chat", service_tx).await?;
            }
            KeyCode::Char('x') => {
              self.stop_service("signaling", service_tx).await?;
            }
            KeyCode::Char('z') => {
              self.stop_service("chat", service_tx).await?;
            }
            _ => {}
          }
        } else {
          self.tui.handle_key_event(key)?;
        }
      }

      AppEvent::Service(service_event) => {
        self.handle_service_event(service_event).await?;
      }

      AppEvent::Tick => {
        // Обновляем статусы сервисов
        self.update_service_statuses();
      }
    }

    Ok(false)
  }

  async fn start_service(
    &mut self,
    service_name: &str,
    _service_tx: &mpsc::UnboundedSender<ServiceEvent>,
  ) -> anyhow::Result<()> {
    self.status_message = format!("Starting {} service...", service_name);

    // Здесь должна быть логика запуска сервиса через ServiceManager
    // Пока просто обновляем статус
    if let Some(service) = self.services.iter_mut().find(|s| s.name == service_name) {
      service.status = "Starting".to_string();
      service.last_update = Instant::now();
    }

    Ok(())
  }

  async fn stop_service(
    &mut self,
    service_name: &str,
    _service_tx: &mpsc::UnboundedSender<ServiceEvent>,
  ) -> anyhow::Result<()> {
    self.status_message = format!("Stopping {} service...", service_name);

    if let Some(service) = self.services.iter_mut().find(|s| s.name == service_name) {
      service.status = "Stopping".to_string();
      service.last_update = Instant::now();
    }

    Ok(())
  }

  async fn handle_service_event(&mut self, event: ServiceEvent) -> anyhow::Result<()> {
    match event {
      ServiceEvent::Signaling(sig_event) => match sig_event {
        SignalingEvent::ClientConnected { client_id } => {
          self.status_message = format!("Client connected: {}", client_id);
          self.add_chat_message(format!("Client connected: {}", client_id));
        }
        SignalingEvent::ClientDisconnected { client_id } => {
          self.status_message = format!("Client disconnected: {}", client_id);
          self.add_chat_message(format!("Client disconnected: {}", client_id));
        }
        SignalingEvent::MessageForwarded { from, to } => {
          self.add_chat_message(format!("Message forwarded from {} to {}", from, to));
        }
      },
      ServiceEvent::Chat(chat_event) => match chat_event {
        ChatEvent::MessageReceived { from, content } => {
          self.add_chat_message(format!("[{}] {}", from, content));
        }
        ChatEvent::UserJoined { user } => {
          self.status_message = format!("User joined: {}", user);
          self.connected_users.push(user.clone());
          self.add_chat_message(format!("User joined: {}", user));
        }
        ChatEvent::UserLeft { user } => {
          self.status_message = format!("User left: {}", user);
          self.connected_users.retain(|u| u != &user);
          self.add_chat_message(format!("User left: {}", user));
        }
      },
      ServiceEvent::System(sys_event) => match sys_event {
        SystemEvent::ServiceStarted { name } => {
          self.status_message = format!("Service started: {}", name);
          if let Some(service) = self.services.iter_mut().find(|s| s.name == name) {
            service.status = "Running".to_string();
            service.last_update = Instant::now();
          }
        }
        SystemEvent::ServiceStopped { name } => {
          self.status_message = format!("Service stopped: {}", name);
          if let Some(service) = self.services.iter_mut().find(|s| s.name == name) {
            service.status = "Stopped".to_string();
            service.last_update = Instant::now();
          }
        }
        SystemEvent::Error { service, message } => {
          self.status_message = format!("Error in {}: {}", service, message);
          self.add_chat_message(format!("ERROR [{}]: {}", service, message));
        }
      },
      ServiceEvent::Status(ServiceEventMessage { service, message }) => {
        todo!()
      }
      ServiceEvent::Error(ServiceEventMessage { service, message }) => {
        todo!()
      }
    }

    Ok(())
  }

  fn update_service_statuses(&mut self) {
    // Здесь можно добавить логику для обновления статусов сервисов
    // Например, если сервис не отвечает больше 30 секунд - помечать как "Not responding"
    for service in &mut self.services {
      if service.status == "Running" {
        // Проверяем, не завис ли сервис
        if service.last_update.elapsed() > Duration::from_secs(60) {
          service.status = "Not responding".to_string();
        }
      }
    }
  }

  fn add_chat_message(&mut self, message: String) {
    self.chat_messages.push(message);
    // Ограничиваем количество сообщений для производительности
    if self.chat_messages.len() > 1000 {
      self.chat_messages.drain(0..100);
    }
  }
}
