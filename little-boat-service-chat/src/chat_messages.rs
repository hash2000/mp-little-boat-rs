use async_trait::async_trait;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use little_boat_abstractions::{ChatMessage, ChatPeerConnections};
use little_boat_abstractions::{
  ControlEvent, IConfigReader, IService, ServiceEvent, SignalingEvent, SignalingMessage,
  SignalingPeers, SystemEvent,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use uuid::Uuid;

use crate::ChatService;

impl ChatService {
    pub async fn send_message(&self, content: &str, from: &str) ->  anyhow::Result<()> {
        let pcs = self.peer_connections.lock().await;
        
        if let Some(pc) = pcs.get("main") {
            let chat_msg = ChatMessage::Text {
                from: from.to_string(),
                content: content.to_string(),
            };
            
            if let Ok(json) = serde_json::to_string(&chat_msg) {
                // here need take DataChannel and send message
                little_boat_abstractions::log_info!("chat", "Message queued: {}", content);
            }
        }
        
        Ok(())
    }
    
    pub async fn get_peers(&self) -> Vec<String> {
        let pcs = self.peer_connections.lock().await;
        pcs.keys().cloned().collect()
    }
}