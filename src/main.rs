use std::env;
use dotenvy::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::PgPool;

mod state;
mod handlers;
mod server;
mod client;
mod database;

use crate::database::connect_db;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env (e.g. postgres://user:pass@localhost:5432/chat_app)");

    let pool: PgPool = connect_db(&db_url).await;

    let clients = Arc::new(Mutex::new(Vec::new()));

    let state = AppState {
        clients: clients.clone(),
        db: pool.clone(),
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  cargo run -- start      # start server");
        println!("  cargo run -- connect    # start a CLI client");
        return;
    }

    match args[1].as_str() {
        "start" => {
            server::run_server(state).await;
        }
        "connect" => {
            client::run_client().await;
        }
        _ => {
            println!("Unknown command");
        }
    }
}