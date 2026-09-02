use crate::state::AppState;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};

use futures_util::{SinkExt, StreamExt};
use sqlx::Row;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(socket: WebSocket, state: AppState) {
    let (sender, mut receiver) = socket.split();

    {
        let mut clients = state.clients.lock().await;
        clients.push(sender);
    }

    let mut user_id: Option<i32> = None;

    while let Some(result) = receiver.next().await {
        match result {
            Ok(msg) => {
                match msg.clone() {
                    Message::Text(text) => {
                        if user_id.is_none() {
                            let pool = &state.db;

                            let row = sqlx::query(
                                r#"
                                INSERT INTO users (username)
                                VALUES ($1)
                                ON CONFLICT (username)
                                DO UPDATE SET username = EXCLUDED.username
                                RETURNING id
                                "#
                            )
                            .bind(&text)
                            .fetch_one(pool)
                            .await;

                            if let Ok(record) = row {
                                let id: i32 = record.get("id");
                                user_id = Some(id);
                            }

                            continue;
                        }

                        let uid = user_id.unwrap();
                        let pool = &state.db;

                        let _ = sqlx::query(
                            "INSERT INTO messages (user_id, content) VALUES ($1, $2)"
                        )
                        .bind(uid)
                        .bind(&text)
                        .execute(pool)
                        .await;
                    }
                    _ => continue,
                }

                let mut clients = state.clients.lock().await;

                let mut i = 0;

                while i < clients.len() {
                    if clients[i].send(msg.clone()).await.is_err() {
                        let _ = clients.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
            Err(_) => break,
        }
    }
}