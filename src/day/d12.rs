use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use chrono::Datelike;
use serde::Serialize;
use tokio::time::Instant;
use ulid::Ulid;
use uuid::Uuid;

use crate::AppState;

pub async fn save_string(State(state): State<AppState>, Path(string): Path<String>) {
    state
        .save_string
        .lock()
        .unwrap()
        .insert(string, Instant::now());
}

pub async fn load_string(
    State(state): State<AppState>,
    Path(string): Path<String>,
) -> Result<String, StatusCode> {
    let lock = state.save_string.lock().unwrap();
    let instant = lock.get(&string).ok_or(StatusCode::NOT_ACCEPTABLE)?;

    Ok(instant.elapsed().as_secs().to_string())
}

pub async fn ulids(Json(payload): Json<Vec<Ulid>>) -> Json<Vec<Uuid>> {
    let response: Vec<Uuid> = payload
        .iter()
        .rev()
        .map(|id| Uuid::from_bytes(id.to_bytes()))
        .collect();

    Json(response)
}

#[derive(Serialize, Default)]
pub struct Weekday {
    #[serde(rename(serialize = "christmas eve"))]
    christmas_eve: u16,
    weekday: u16,
    #[serde(rename(serialize = "in the future"))]
    in_the_future: u16,
    #[serde(rename(serialize = "LSB is 1"))]
    lsb: u16,
}

pub async fn ulids_weekday(
    Path(weekday): Path<u8>,
    Json(payload): Json<Vec<Ulid>>,
) -> Json<Weekday> {
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
