use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::state::AppState;
use crate::models::users::{signup, login};
use crate::handlers::ws_handler::ws_handler;
use crate::middleware::auth::auth_middleware;

pub async fn run(state: Arc<AppState>) {
    let app = create_router(state.clone());

    let addr = "127.0.0.1:3000";
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(_) => return,
    };

    println!("server running on {}", addr);

    if axum::serve(listener, app).await.is_err() {
        return;
    }
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
        .route("/ws", get(ws_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .merge(protected_routes)
        .with_state(state)
}