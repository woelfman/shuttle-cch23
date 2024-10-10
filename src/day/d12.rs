//! Day 12: Timekeeper
//!
//! One frosty night, Santa, dressed warmly in his favorite red coat, decided to
//! take a midnight stroll around the elf workshop. As he pushed open the heavy
//! wooden doors of the workshop, his eyes widened in surprise. He was
//! completely stunned by the sight that greeted him.
//!
//! Rows upon rows of conveyor belts had been set up, zipping toys from one
//! corner to the other, resembling an intricate dance of festivity and
//! efficiency. The elves were operating with military precision, organizing
//! toys into specific categories and sending them down the right pathways.
//!
//! # Task 1: How To time Persist? (HTTP)
//!
//! Presents are being packed and wrapped at blazingly fast speeds in the
//! workshop. In order to gather data on the production of presents, Santa needs
//! a multi-stopwatch that can keep the time of many packet IDs at once.
//!
//! Create two endpoints:
//!
//! * POST `/12/save/<string>`: takes a string and stores it.
//! * GET `/12/load/<string>`: takes the same string and returns the number of
//!   whole seconds elapsed since the last time it was stored.
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/12/save/packet20231212
//! sleep 2
//! curl http://localhost:8000/12/load/packet20231212
//! echo
//! sleep 2
//! curl http://localhost:8000/12/load/packet20231212
//! echo
//! curl -X POST http://localhost:8000/12/save/packet20231212
//! curl http://localhost:8000/12/load/packet20231212
//!
//! # After ~4 seconds:
//! 2
//! 4
//! 0
//! ```
//!
//! # Task 2: Unanimously Legendary IDentifier (ULID)
//!
//! Santa, who likes old-school tech, now sees that some packets use modern
//! ULIDs. Help him rewind time a little bit by showing him them in an older
//! format that he understands.
//!
//! Make a POST endpoint `/12/ulids` that takes a JSON array of ULIDs. Convert
//! all the ULIDs to UUIDs and return a new array but in reverse order.
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/12/ulids \
//!   -H 'Content-Type: application/json' \
//!   -d '[
//!     "01BJQ0E1C3Z56ABCD0E11HYX4M",
//!     "01BJQ0E1C3Z56ABCD0E11HYX5N",
//!     "01BJQ0E1C3Z56ABCD0E11HYX6Q",
//!     "01BJQ0E1C3Z56ABCD0E11HYX7R",
//!     "01BJQ0E1C3Z56ABCD0E11HYX8P"
//!   ]'
//!
//! [
//!   "015cae07-0583-f94c-a5b1-a070431f7516",
//!   "015cae07-0583-f94c-a5b1-a070431f74f8",
//!   "015cae07-0583-f94c-a5b1-a070431f74d7",
//!   "015cae07-0583-f94c-a5b1-a070431f74b5",
//!   "015cae07-0583-f94c-a5b1-a070431f7494"
//! ]
//! ```
//!
//! # Task 3: Let Santa Broil (LSB)
//!
//! Now that Santa is up to date on some newer data formats, he needs help with
//! analyzing the manufacturing date of some packets he found in the corner of
//! the workshop.
//!
//! Create another variant of the same endpoint `/12/ulids/<weekday>` that
//! counts the number of ULIDs that fulfill the following criteria (in the UTC
//! timezone):
//!
//! How many of the ULIDs were generated on a Christmas Eve?  How many were
//! generated on a `<weekday>`? (A number in the path between 0 (Monday) and 6
//! (Sunday)) How many were generated in the future? (has a date later than the
//! current time) How many have entropy bits where the Least Significant Bit
//! (LSB) is 1?
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/12/ulids/5 \
//!   -H 'Content-Type: application/json' \
//!   -d '[
//!     "00WEGGF0G0J5HEYXS3D7RWZGV8",
//!     "76EP4G39R8JD1N8AQNYDVJBRCF",
//!     "018CJ7KMG0051CDCS3B7BFJ3AK",
//!     "00Y986KPG0AMGB78RD45E9109K",
//!     "010451HTG0NYWMPWCEXG6AJ8F2",
//!     "01HH9SJEG0KY16H81S3N1BMXM4",
//!     "01HH9SJEG0P9M22Z9VGHH9C8CX",
//!     "017F8YY0G0NQA16HHC2QT5JD6X",
//!     "03QCPC7P003V1NND3B3QJW72QJ"
//!   ]'
//!
//! {
//!   "christmas eve": 3,
//!   "weekday": 1,
//!   "in the future": 2,
//!   "LSB is 1": 5
//! }
//! ```
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use chrono::Datelike;
use serde::Serialize;
use tokio::time::Instant;
use ulid::Ulid;
use uuid::Uuid;

/// Get Day 12 routes
///
/// * `/12/save/<string>`
/// * `/12/load/<string>`
/// * `/12/ulids`
/// * `/12/load/<weekday>`
pub fn get_routes() -> Router {
    let state = AppState {
        save_string: Arc::new(Mutex::new(HashMap::new())),
    };

    Router::new()
        .route("/12/save/:string", post(save_string))
        .route("/12/load/:string", get(load_string))
        .route("/12/ulids", post(ulids))
        .route("/12/ulids/:weekday", post(ulids_weekday))
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    pub save_string: Arc<Mutex<HashMap<String, Instant>>>,
}

async fn save_string(State(state): State<AppState>, Path(string): Path<String>) {
    state
        .save_string
        .lock()
        .unwrap()
        .insert(string, Instant::now());
}

async fn load_string(
    State(state): State<AppState>,
    Path(string): Path<String>,
) -> Result<String, StatusCode> {
    let lock = state.save_string.lock().unwrap();
    let instant = lock.get(&string).ok_or(StatusCode::NOT_ACCEPTABLE)?;

    Ok(instant.elapsed().as_secs().to_string())
}

async fn ulids(Json(payload): Json<Vec<Ulid>>) -> Json<Vec<Uuid>> {
    let response: Vec<Uuid> = payload
        .iter()
        .rev()
        .map(|id| Uuid::from_bytes(id.to_bytes()))
        .collect();

    Json(response)
}

#[derive(Serialize, Default)]
struct Weekday {
    #[serde(rename(serialize = "christmas eve"))]
    christmas_eve: u16,
    weekday: u16,
    #[serde(rename(serialize = "in the future"))]
    in_the_future: u16,
    #[serde(rename(serialize = "LSB is 1"))]
    lsb: u16,
}

async fn ulids_weekday(Path(weekday): Path<u8>, Json(payload): Json<Vec<Ulid>>) -> Json<Weekday> {
    let mut response = Weekday::default();

    for id in payload {
        let ts = chrono::DateTime::from_timestamp_millis(id.timestamp_ms() as i64).unwrap();
        if ts.month() == 12 && ts.day() == 24 {
            response.christmas_eve += 1;
        }

        if ts.weekday() as u8 == weekday {
            response.weekday += 1;
        }

        if ts.timestamp() > chrono::Utc::now().timestamp() {
            response.in_the_future += 1;
        }

        if id.random() & 1 == 1 {
            response.lsb += 1;
        }
    }

    Json(response)
}

#[cfg(test)]
mod test {
    use super::*;
    use axum_test::TestServer;
    use serde_json::json;
    use std::time::Duration;

    #[tokio::test]
    async fn test_task1() {
        let app = get_routes();

        let server = TestServer::new(app).unwrap();

        let response = server.post("/12/save/packet20231212").await;
        response.assert_status(StatusCode::OK);
        tokio::time::sleep(Duration::from_secs(1)).await;
        let response = server.get("/12/load/packet20231212").await;
        response.assert_text("1");
        tokio::time::sleep(Duration::from_secs(1)).await;
        let response = server.get("/12/load/packet20231212").await;
        response.assert_text("2");
        tokio::time::sleep(Duration::from_secs(1)).await;
        let response = server.get("/12/load/packet20231212").await;
        response.assert_text("3");
    }

    #[tokio::test]
    async fn test_task2() {
        let app = get_routes();

        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/12/ulids")
            .json(&json!([
                "01BJQ0E1C3Z56ABCD0E11HYX4M",
                "01BJQ0E1C3Z56ABCD0E11HYX5N",
                "01BJQ0E1C3Z56ABCD0E11HYX6Q",
                "01BJQ0E1C3Z56ABCD0E11HYX7R",
                "01BJQ0E1C3Z56ABCD0E11HYX8P"
            ]))
            .await;
        response.assert_json(&json!([
            "015cae07-0583-f94c-a5b1-a070431f7516",
            "015cae07-0583-f94c-a5b1-a070431f74f8",
            "015cae07-0583-f94c-a5b1-a070431f74d7",
            "015cae07-0583-f94c-a5b1-a070431f74b5",
            "015cae07-0583-f94c-a5b1-a070431f7494"
        ]));
    }

    #[tokio::test]
    async fn test_task3() {
        let app = get_routes();

        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/12/ulids/5")
            .json(&json!([
                "00WEGGF0G0J5HEYXS3D7RWZGV8",
                "76EP4G39R8JD1N8AQNYDVJBRCF",
                "018CJ7KMG0051CDCS3B7BFJ3AK",
                "00Y986KPG0AMGB78RD45E9109K",
                "010451HTG0NYWMPWCEXG6AJ8F2",
                "01HH9SJEG0KY16H81S3N1BMXM4",
                "01HH9SJEG0P9M22Z9VGHH9C8CX",
                "017F8YY0G0NQA16HHC2QT5JD6X",
                "03QCPC7P003V1NND3B3QJW72QJ"
            ]))
            .await;
        response.assert_json(&json!({
            "christmas eve": 3,
            "weekday": 1,
            "in the future": 2,
            "LSB is 1": 5
        }));
    }
}
