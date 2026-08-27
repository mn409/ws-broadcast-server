use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::state::AppState;
use crate::handlers::ws_handler;

pub async fn run_server() {
    let state = AppState {
        clients: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on ws://127.0.0.1:3000/ws");

    axum::serve(listener, app).await.unwrap();
}