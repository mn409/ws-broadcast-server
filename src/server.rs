use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

mod models {
    pub mod users {
        pub async fn signup() {}
        pub async fn login() {}
    }
}

use crate::state::AppState;
use crate::models::users::{signup, login};
use crate::handlers::ws_handler::ws_handler;

pub async fn run_server(state: AppState) {
    let app = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}