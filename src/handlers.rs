use crate::state::AppState;
use axum::{
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(socket: WebSocket, state: AppState) {
    let (sender, mut receiver) = socket.split();

    {
        let mut clients = state.clients.lock().await;
        clients.push(sender);
    }

    while let Some(result) = receiver.next().await {
        match result {
            Ok(msg) => {
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
