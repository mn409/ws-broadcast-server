use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use url::Url;
use tokio::io::{self, AsyncBufReadExt};

pub async fn run_client() {
    let url = Url::parse("ws://127.0.0.1:3000/ws").unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("Connected to server");

    let (mut write, mut read) = ws_stream.split();

    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            if let Ok(msg) = msg {
                println!("Received: {}", msg);
            }
        }
    });

    let stdin = io::BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        if write.send(line.into()).await.is_err() {
            println!("Failed to send");
            break;
        }
    }
}