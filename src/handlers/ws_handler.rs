use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::collections::HashMap;
use futures::{StreamExt, SinkExt};

use crate::state::AppState;
use crate::model::jwt::verify_token;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> impl IntoResponse {

    let token = match params.get("token") {
        Some(t) => t,
        None => {
            return (StatusCode::UNAUTHORIZED, "Missing token").into_response();
        }
    };

    let claims = match verify_token(token) {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }
    };

    ws.on_upgrade(move |socket| handle_socket(socket, claims.sub, state))
}

async fn handle_socket(
    socket: WebSocket,
    user_id: String,
    state: AppState,
) {
    let (mut sender, mut receiver) = socket.split();

    println!("User connected: {}", user_id);

    while let Some(Ok(msg)) = receiver.next().await {
        if let Ok(text) = msg.to_text() {
            println!("{}: {}", user_id, text);
        }
    }

    println!("User disconnected: {}", user_id);
}