use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::{
    state::AppState,
    models::users::{signup, login},
    handlers::ws_handler::ws_handler,
    middleware::auth::auth_middleware,
};

pub async fn run(state: Arc<AppState>) {
    let app = create_router(state);

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("server running on {}", addr);

    axum::serve(listener, app).await.unwrap();
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let protected = Router::new()
        .route("/ws", get(ws_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .merge(protected)
        .with_state(state)
}