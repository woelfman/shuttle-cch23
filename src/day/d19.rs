use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use futures_util::{SinkExt, StreamExt};
use postage::{broadcast::Sender, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Routes supported by this day
///
/// This is new starting with 19, as the global state is beginning to get messy.
/// This way state variables the route depends on are localized and not exposed
/// to other days that shouldn't care about these state variables.
pub fn get_routes() -> Router {
    let state = PongState::Init;

    let ping_router = Router::new()
        .route("/19/ws/ping", get(ping))
        .with_state(state);

    let state = Arc::new(RwLock::new(BirdApp::default()));

    let bird_app_router = Router::new()
        .route("/19/reset", post(reset))
        .route("/19/views", get(views))
        .route("/19/ws/room/:room/user/:user", get(room))
        .with_state(state);

    Router::new()
        .nest("/", ping_router)
        .nest("/", bird_app_router)
}

async fn ping(ws: WebSocketUpgrade, State(state): State<PongState>) -> Response {
    ws.on_upgrade(|socket| handle_ping(socket, state))
}

#[derive(Clone)]
enum PongState {
    Init,
    Started,
}

async fn handle_ping(mut socket: WebSocket, mut state: PongState) {
    while let Some(Ok(msg)) = socket.recv().await {
        match state {
            PongState::Init => {
                if let Message::Text(msg) = msg {
                    if msg == "serve" {
                        state = PongState::Started;
                    }
                }
            }
            PongState::Started => {
                if let Message::Text(msg) = msg {
                    if msg == "ping" {
                        socket
                            .send("pong".into())
                            .await
                            .expect("Failed to send pong");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct BirdApp {
    views: u64,
    rooms: HashMap<u64, Sender<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Tweet {
    message: String,
}

async fn reset(State(state): State<Arc<RwLock<BirdApp>>>) {
    state.write().unwrap().views = 0;
}

async fn views(State(state): State<Arc<RwLock<BirdApp>>>) -> impl IntoResponse {
    let views = state.read().unwrap().views.to_string();
    tracing::info!("views: {views}");
    views
}

async fn room(
    ws: WebSocketUpgrade,
    State(state): State<Arc<RwLock<BirdApp>>>,
    Path((room, user)): Path<(u64, String)>,
) -> Response {
    ws.on_upgrade(move |socket| handle_room(socket, state, room, user))
}

async fn handle_room(ws: WebSocket, state: Arc<RwLock<BirdApp>>, room: u64, user: String) {
    let (mut sender, mut receiver) = ws.split();

    let mut room_sender = state
        .write()
        .unwrap()
        .rooms
        .entry(room)
        .or_insert(postage::broadcast::channel(32).0)
        .clone();

    let mut room_subscriber = room_sender.subscribe();

    let mut task_sender = tokio::spawn(async move {
        while let Some(msg) = room_subscriber.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
            state.write().unwrap().views += 1;
        }
    });

    let mut task_receiver = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            let tweet: Tweet = serde_json::from_str(&msg).expect("Invalid JSON message");
            if tweet.message.chars().count() > 128 {
                continue;
            }
            let msg = json!({"user": user, "message": tweet.message}).to_string();
            let _ = room_sender.send(msg).await;
        }
    });

    tokio::select! {
        _ = (&mut task_sender) => task_sender.abort(),
        _ = (&mut task_receiver) => task_receiver.abort(),
    }
}
