mod services;
mod config;

pub use crate::services::{
  // types of system events
  SystemEvent,

  // types of controling events
  ControlEvent,

  // types of service events
  ServiceEvent,
  ServiceEventMessage,

  // types of chat service
  ChatEvent,
  ChatPeerConnections,
  ChatSocketSender,

  // types of signaling service
  SignalingEvent,
  SignalingMessage,
  SignalingPeers,
  
  // service trait
  IService,
};

pub use crate::config::{
  IConfigReader,
  IConfigWriter,
  IConfig,
};


#[macro_export]
macro_rules! log_info {
  ($service:expr, $msg:expr) => {
    log::info!("[{}] {}", $service, $msg);
  };
  ($service:expr, $fmt:expr, $($arg:tt)*) => {
    log::info!("[{}] {}", $service, format!($fmt, $($arg)*));
  };
}

#[macro_export]
macro_rules! log_error {
  ($service:expr, $msg:expr) => {
    log::error!("[{}] {}", $service, $msg);
  };
  ($service:expr, $fmt:expr, $($arg:tt)*) => {
    log::error!("[{}] {}", $service, format!($fmt, $($arg)*));
  };
}

#[macro_export]
macro_rules! log_warn {
  ($service:expr, $msg:expr) => {
    log::warn!("[{}] {}", $service, $msg);
  };
  ($service:expr, $fmt:expr, $($arg:tt)*) => {
    log::warn!("[{}] {}", $service, format!($fmt, $($arg)*));
  };
}