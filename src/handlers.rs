use crate::state::AppState;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};

use futures_util::{SinkExt, StreamExt};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(socket: WebSocket, state: AppState) {
    println!("HANDLER STARTED");

    let (sender, mut receiver) = socket.split();

    {
        let mut clients = state.clients.lock().await;
        clients.push(sender);
    }

    while let Some(result) = receiver.next().await {
        match result {
            Ok(msg) => {
                println!("Received msg: {:?}", msg);

                match msg.clone() {
                    Message::Text(text) => {
                        println!("TEXT MESSAGE DETECTED: {}", text);

                        let pool = &state.db;

                        let result =
                            sqlx::query("INSERT INTO messages (content) VALUES ($1)")
                                .bind(&text)
                                .execute(pool)
                                .await;

                        match result {
                            Ok(_) => {
                                println!("INSERT SUCCESS");
                            }

                            Err(e) => {
                                println!("DB INSERT ERROR: {:?}", e);
                            }
                        }
                    }

                    other => {
                        println!("NOT TEXT MESSAGE: {:?}", other);
                    }
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

            Err(e) => {
                println!("WEBSOCKET RECEIVE ERROR: {:?}", e);
                break;
            }
        }
    }

    println!("CLIENT DISCONNECTED");
}