use std::env;

mod state;
mod handlers;
mod server;
mod client;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("broadcast-server start");
        println!("broadcast-server connect");
        return;
    }

    match args[1].as_str() {
        "start" => {
            server::run_server().await;
        }
        "connect" => {
            client::run_client().await;
        }
        _ => {
            println!("Unknown command");
        }
    }
}