use crate::views::{ChatBoardView, View};

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use little_boat_abstractions::ServiceEvent;
use little_boat_core::ClientApp;
use ratatui::{Frame, layout::Rect};

pub struct Application {
  view: Box<dyn View>,
  exit: bool,
  client: ClientApp,
}

impl Application {
  pub fn new() -> anyhow::Result<Self> {
    let app = Self { 
      view: Box::new(ChatBoardView::new()), 
      exit: false,
      client: ClientApp::new()?,
    };

    Ok(app)
  }

  pub async fn init(&mut self) -> anyhow::Result<()> {
    self.client.serve("signaling").await?;
    self.client.serve("chat").await?;

    Ok(())
  }

  pub fn draw(&mut self, f: &mut Frame) {
    self.view.draw(f, Rect::ZERO);
  }

  pub fn set_view(&mut self, view: Box<dyn View>) {
    self.view = view;
  }

  pub fn begin_frame(&mut self) {}

  pub fn handle_service_event(&mut self, event: &ServiceEvent) {
    self.view.handle_service_event(event);
  }

  pub fn append_event(&mut self, event: &Event) {
    if let Event::Key(key) = event {
      if key.kind == KeyEventKind::Press {
        match key.code {
          KeyCode::Char('q') => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
              self.exit = true
            }
          }
          _ => (),
        }
      }
    }

    self.view.handle_event(event);
  }

  pub fn exit(&self) -> bool {
    self.exit
  }

  pub async fn shutdown(&mut self) -> anyhow::Result<()> {
    self.client.shutdown().await
  }
}
