use std::sync::Arc;
use tokio::sync::Mutex;
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;

pub type Sender = SplitSink<WebSocket, Message>;

#[derive(Clone)]
pub struct AppState {
    pub clients: Arc<Mutex<Vec<Sender>>>,
}