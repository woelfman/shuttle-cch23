//! Day 19: Christmas Sockets on the Chimney
//!
//! On a cold and snowy winter day, Santa Claus was busy with his annual routine
//! when he spotted a new delivery of vibrant socks hanging on his chimney. The
//! hues and prints on these socks were unlike anything he had seen before -
//! intricate patterns with tiny paddles embroidered on them. He chuckled,
//! remembering how he used to juggle between writing protocols for his
//! websocket apps and practising his backhand strokes on his virtual table
//! tennis game.
//!
//! # Task 1: Table Tennis Server
//!
//! Write a WebSocket GET endpoint `/19/ws/ping` that listens for messages of
//! type Text.
//!
//! * If the incoming string is `serve`, the game starts in this WebSocket.
//! * If and only if the game has started, respond with a string `pong` whenever
//!   the incoming string is `ping`.
//! * All other incoming messages should be ignored.
//!
//! # Task 2: Bird App Simulator
//!
//! To improve internal communications at the North Pole, Santa is trying out a
//! real-time variant of Twitter (sometimes referred to as a "chat app"). *(Santa
//! is old-school & cool - still calls it Twitter instead of X)*.
//!
//! In order to know how much the elves are using the platform, Santa wants some
//! metrics. He thinks it is sufficient to just count the total number of views
//! on all tweets.
//!
//! Here are the required endpoints:
//!
//! * POST endpoint `/19/reset` that resets the counter of tweet views.
//! * GET endpoint `/19/views` that returns the current count of tweet views.
//! * GET endpoint `/19/ws/room/<number>/user/<string>` that opens a WebSocket
//!   and connects a user to a room.
//!
//! This is how the app should work:
//!
//! * A user can at any time send a tweet as a Text WebSocket message in the
//!   format `{"message":"Hello North Pole!"}`.
//! * When a tweet is received, broadcast it to everyone in the same room
//!   (including the sender).
//! * Tweets with more than 128 characters are too long and should be ignored by
//!   the server.
//! * Tweets sent out to room members should have the format
//!   `{"user":"xX_f4th3r_0f_chr1stm4s_Xx","message":"Hello North Pole!"}` where
//!   user is the author of the tweet (the username that the sender used in the
//!   endpoint's URL path).
//! * Every time a tweet is successfully sent out to a user, it counts as one
//!   view.
//! * Keep a running count of the number of views that happen, and return the
//!   current view count from the `/19/views` endpoint whenever requested.
//! * When a websocket closes, that user leaves the room and should no longer
//!   receive tweets.
//! * When the reset endpoint is called, the counter is set to 0.
//!
//! The view counter can be in-memory and does not need to persist.
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
