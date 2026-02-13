use std::sync::OnceLock;

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Query},
    response::IntoResponse,
};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::mpsc;
use uuid::Uuid;

type Sender = mpsc::UnboundedSender<String>;
type Connections = DashMap<Uuid, Vec<Sender>>;

static WS_CONNECTIONS: OnceLock<Connections> = OnceLock::new();

fn connections() -> &'static Connections {
    WS_CONNECTIONS.get_or_init(DashMap::new)
}

/// Broadcast a JSON message to a specific user's open WebSocket connections.
pub fn broadcast_to_user(user_id: Uuid, message: &str) {
    if let Some(mut senders) = connections().get_mut(&user_id) {
        senders.retain(|tx| tx.send(message.to_string()).is_ok());
    }
}

#[derive(Deserialize)]
pub struct WsParams {
    pub token: String,
}

/// Axum handler for WebSocket upgrade.
/// Connect with: ws://localhost:8080/ws?token=<jwt>
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
) -> impl IntoResponse {
    // Validate the JWT token
    let claims = match crate::auth::validate_token(&params.token) {
        Ok(c) => c,
        Err(_) => {
            return axum::http::Response::builder()
                .status(401)
                .body(axum::body::Body::from("Invalid token"))
                .unwrap()
                .into_response();
        }
    };

    let user_id: Uuid = match claims.sub.parse() {
        Ok(id) => id,
        Err(_) => {
            return axum::http::Response::builder()
                .status(400)
                .body(axum::body::Body::from("Invalid user ID in token"))
                .unwrap()
                .into_response();
        }
    };

    ws.on_upgrade(move |socket| handle_socket(socket, user_id))
        .into_response()
}

async fn handle_socket(socket: WebSocket, user_id: Uuid) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Register this connection
    connections().entry(user_id).or_default().push(tx);

    println!("WebSocket connected: user {user_id}");

    // Task: forward messages from our channel to the WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Task: read from WebSocket (handle pings, keep-alive)
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Close(_) => break,
                _ => {} // ignore other incoming messages for now
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // Clean up: remove closed senders for this user
    if let Some(mut senders) = connections().get_mut(&user_id) {
        senders.retain(|tx| !tx.is_closed());
    }

    println!("WebSocket disconnected: user {user_id}");
}
