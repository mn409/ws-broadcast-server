use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, Extension,
    },
    response::IntoResponse,
};
use futures_util::{StreamExt, SinkExt};
use std::sync::Arc;

use crate::state::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id))
}

async fn handle_socket(
    socket: WebSocket,
    _state: Arc<AppState>,
    user_id: String,
) {
    let (mut sender, mut receiver) = socket.split();

    println!("user connected: {}", user_id);

    while let Some(msg_result) = receiver.next().await {
        let msg = match msg_result {
            Ok(m) => m,
            Err(_) => break,
        };

        match msg {
            Message::Text(text) => {
                let response = format!("{}: {}", user_id, text);

                if sender.send(Message::Text(response)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    println!("user disconnected: {}", user_id);
}