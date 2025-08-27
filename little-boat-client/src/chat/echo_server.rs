use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

// Состояние приложения (может быть пустым в данном случае)
#[derive(Clone)]
struct AppState;

pub async fn run() {
    let app_state = Arc::new(AppState);

    let app = Router::new()
        .route("/", get(websocket_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030")
        .await
        .unwrap();
    println!("WebSocket server running on ws://127.0.0.1:3030");
    
    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Задача для отправки сообщений из канала в WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(message).await.is_err() {
                break;
            }
        }
    });

    // Задача для приема сообщений из WebSocket и отправки в канал
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(msg, &tx).await {
                break;
            }
        }
    });

    // Ожидаем завершения одной из задач
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}

async fn process_message(msg: Message, tx: &mpsc::UnboundedSender<Message>) -> bool {
    match msg {
        Message::Text(text) => {
            // Эхо-ответ текстового сообщения
            if let Err(e) = tx.send(Message::Text(text)) {
                eprintln!("Failed to send message: {}", e);
                return true;
            }
        }
        Message::Binary(data) => {
            // Эхо-ответ бинарного сообщения
            if let Err(e) = tx.send(Message::Binary(data)) {
                eprintln!("Failed to send message: {}", e);
                return true;
            }
        }
        Message::Ping(data) => {
            // Автоматически отвечаем на Pong
            if let Err(e) = tx.send(Message::Pong(data)) {
                eprintln!("Failed to send pong: {}", e);
                return true;
            }
        }
        Message::Pong(_) => {
            // Игнорируем Pong сообщения
        }
        Message::Close(_) => {
            // Закрываем соединение
            return true;
        }
    }
    false
}