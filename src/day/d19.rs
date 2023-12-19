use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

/// Routes supported by this day
///
/// This is new starting with 19, as the global state is beginning to get messy.
/// This way state variables the route depends on are localized and not exposed
/// to other days that shouldn't care about these state variables.
pub fn get_routes() -> Router {
    Router::new()
        .route("/19/ws/ping", get(ping))
        .route("/19/reset", post(reset))
        .route("/19/views", get(views))
        .route("/19/ws/room/:room/user/:user", get(room))
        .with_state(BirdApp::default())
}

async fn ping(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_ping)
}

enum PongState {
    Init,
    Started,
}

async fn handle_ping(mut socket: WebSocket) {
    let mut state = PongState::Init;

    while let Some(msg) = socket.recv().await {
        match state {
            PongState::Init => {
                if let Ok(msg) = msg {
                    if matches!(msg.to_text(), Ok("serve")) {
                        state = PongState::Started;
                    }
                } else {
                    // client disconnected
                    return;
                };
            }
            PongState::Started => {
                if let Ok(msg) = msg {
                    if matches!(msg.to_text(), Ok("ping"))
                        && socket.send("pong".into()).await.is_err()
                    {
                        // client disconnected
                        return;
                    }
                } else {
                    // client disconnected
                    return;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct BirdApp {
    views: Arc<Mutex<u64>>,
    rooms: Arc<Mutex<HashMap<u64, Room>>>,
}

type Room = HashMap<String, UserSink>;
type UserSink = Arc<Mutex<SplitSink<WebSocket, Message>>>;

#[derive(Serialize, Deserialize, Clone)]
struct Tweet {
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    message: String,
}

async fn reset(State(state): State<BirdApp>) {
    *state.views.lock().await = 0;
}

async fn views(State(state): State<BirdApp>) -> impl IntoResponse {
    state.views.lock().await.to_string()
}

async fn room(
    ws: WebSocketUpgrade,
    State(state): State<BirdApp>,
    Path((room, user)): Path<(u64, String)>,
) -> Response {
    ws.on_upgrade(move |socket| handle_room(socket, state, room, user))
}

async fn handle_room(ws: WebSocket, state: BirdApp, room: u64, user: String) {
    let (sender, receiver) = ws.split();

    state
        .rooms
        .lock()
        .await
        .entry(room)
        .or_default()
        .insert(user.clone(), Arc::new(Mutex::new(sender)));

    tokio::spawn(read(receiver, state, room, user));
}

async fn read(mut receiver: SplitStream<WebSocket>, state: BirdApp, room: u64, user: String) {
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if let Ok(mut tweet) = serde_json::from_slice::<Tweet>(&msg.into_data()) {
                if tweet.message.chars().count() > 128 {
                    continue;
                }

                tweet.user = Some(user.clone());
                if let Some(room) = state.rooms.lock().await.get(&room) {
                    for user_sender in room.values() {
                        let _ = user_sender
                            .lock()
                            .await
                            .send(serde_json::to_string(&tweet).unwrap().into())
                            .await;
                    }

                    *state.views.lock().await += room.len() as u64;
                }
            }
        }
    }

    if let Some(room) = state.rooms.lock().await.get_mut(&room) {
        room.remove(&user);
    }
}
