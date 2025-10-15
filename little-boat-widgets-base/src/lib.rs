#![allow(clippy::large_enum_variant, clippy::too_many_arguments)]
pub use self::appearance::Theme;
pub use self::buffer::Buffer;

pub mod appearance;
pub mod compression;
pub mod environment;
pub mod dashboard;
pub mod serde;
pub mod config;
pub mod pane;
pub mod buffer;
pub mod target;
pub mod bouncer;
pub mod server;
pub mod channel;
pub mod shortcut;