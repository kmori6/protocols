use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::any,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct WsState {
    pub tx: broadcast::Sender<String>,
}

impl WsState {
    pub fn new() -> Arc<Self> {
        let (tx, _) = broadcast::channel(64);
        Arc::new(Self { tx })
    }
}

pub fn router() -> Router<Arc<WsState>> {
    Router::new()
        .route("/echo", any(echo_handler))
        .route("/chat", any(chat_handler))
}

// ── echo ──────────────────────────────────────────────────────────────────────

async fn echo_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_echo)
}

async fn handle_echo(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if socket.send(Message::Text(text)).await.is_err() {
                    return;
                }
            }
            Message::Binary(bytes) => {
                if socket.send(Message::Binary(bytes)).await.is_err() {
                    return;
                }
            }
            Message::Close(_) => return,
            _ => {}
        }
    }
}

// ── chat (broadcast) ──────────────────────────────────────────────────────────

async fn chat_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_chat(socket, state))
}

async fn handle_chat(socket: WebSocket, state: Arc<WsState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // Receive messages from the client and publish them to the broadcast channel
    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    let _ = tx.send(text.to_string());
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // Subscribe to the broadcast channel and forward messages to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(text) = rx.recv().await {
            if sender.send(Message::Text(text.into())).await.is_err() {
                break;
            }
        }
    });

    // If either task finishes, abort the other
    tokio::select! {
        _ = &mut recv_task => send_task.abort(),
        _ = &mut send_task => recv_task.abort(),
    }
}
