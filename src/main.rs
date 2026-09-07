use std::env;
use dotenvy::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::PgPool;

mod server;
mod handlers;
mod models;
mod state;
mod database;
mod middleware;

use crate::database::connect_db;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool: PgPool = connect_db(&db_url).await;

    let clients = Arc::new(Mutex::new(Vec::new()));

    let state = Arc::new(AppState {
        clients: clients.clone(),
        db: pool.clone(),
    });

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  cargo run -- start");
        return;
    }

    match args[1].as_str() {
        "start" => {
            server::run(state).await;
        }
        _ => {
            println!("Unknown command");
        }
    }
}