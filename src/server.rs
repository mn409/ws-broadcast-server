use axum::{routing::get, Router};
use crate::state::AppState;
use crate::handlers::ws_handler;

pub async fn run_server(state: AppState) {
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("Server running on ws://127.0.0.1:3000/ws");

    axum::serve(listener, app).await.unwrap();
}