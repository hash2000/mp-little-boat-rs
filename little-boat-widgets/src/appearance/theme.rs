use little_boat_widgets_base as base;

pub use base::appearance::theme::{
  Buffer, Button, Buttons, General, ServerMessages, 
  Styles, Text, color_to_hex, hex_to_color,
};

pub mod container;
pub mod context_menu;
pub mod selectable_text;
pub mod text;

// TODO: If we use non-standard font sizes, we should consider
// Config.font.size since it's user configurable
pub const TEXT_SIZE: f32 = 13.0;
pub const ICON_SIZE: f32 = 12.0;

#[derive(Debug, Clone)]
pub enum Theme {
  Selected(base::Theme),
  Preview { 
    selected: base::Theme, 
    preview: base::Theme },
}

impl Theme {
  pub fn preview(&self, theme: base::Theme) -> Self {
    match self {
      Theme::Selected(selected) | 
      Theme::Preview { selected, .. } => {
        Self::Preview {
          selected: selected.clone(), 
          preview: theme }
      }
    }
  }

  pub fn selected(&self) -> Self {
    match self {
      Theme::Selected(selected) | 
      Theme::Preview { selected, .. } => {
        Self::Selected(selected.clone())
      }
    }
  }

  pub fn styles(&self) -> &Styles {
    match self {
      Theme::Selected(selected) => &selected.styles,
      Theme::Preview { preview, .. } => &preview.styles,
    }
  }
}



impl From<base::Theme> for Theme {
  fn from(theme: base::Theme) -> Self {
    Theme::Selected(theme)
  }
}

impl Default for Theme {
    fn default() -> Self {
        Self::from(base::Theme::default())
    }
}

impl iced::theme::Base for Theme {
  fn base(&self) -> iced::theme::Style {
    iced::theme::Style {
      background_color: self.styles().general.background,
      text_color: self.styles().text.primary.color,
    }
  }

  fn palette(&self) -> Option<iced::theme::Palette> {
    None
  }

  fn default(_preference: iced::theme::Mode) -> Self {
    Self::from(base::Theme::default())
  }

  fn mode(&self) -> iced::theme::Mode {
    iced::theme::Mode::Dark
  }
}