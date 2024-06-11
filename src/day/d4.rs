//! Day 4: What do you call a serialized reindeer? Serdeer!
//!
//! Under the soft glow of the Northern Lights, Santa's reindeer are training
//! for the big night. But, oh deer! The reindeer's stats have been serialized
//! into an unknown format after a playful elf accidentally spilled hot cocoa on
//! the central computer. The data needs to be deserialized so the reindeer team
//! can be assembled based on their combined strength.
//!
//! # Task 1: Reindeer cheer
//!
//! The task is to create a POST endpoint `/4/strength` that calculates the
//! combined strength of a group of reindeer, so that Santa knows if they can
//! pull his sled through the skies.
//!
//! The input to the endpoint is a JSON array containing information about
//! each reindeer. Each reindeer is represented as an object with two
//! attributes: `"name"` (string) and `"strength"` (integer). Collect the strength
//! of each reindeer and respond with the sum.
//!
//! ## Example
//! ```not_rust
//! curl -X POST http://localhost:8000/4/strength \
//!   -H 'Content-Type: application/json' \
//!   -d '[
//!     { "name": "Dasher", "strength": 5 },
//!     { "name": "Dancer", "strength": 6 },
//!     { "name": "Prancer", "strength": 4 },
//!     { "name": "Vixen", "strength": 7 }
//!   ]'
//!
//! 22
//! ```
//!
//! # Task 2: Cursed candy eating contest
//!
//! This time, there is some more data for each reindeer (see example). The
//! endpoint is called `/4/contest`, because the reindeer are now going to
//! compare the attributes of the reindeer and present a summary of the winners.
//!
//! There is at least one reindeer participating in the contest, and there is
//! never a tie for first place.
//!
//! For some reason, one of the field names in the input seems to still be a bit
//! corrupted from the incident. Guess we just have to deal with it anyways.
//!
//! The output should be a JSON object containing a summary of the winners (see
//! example).
//!
//! ## Example Input
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/4/contest \
//!   -H 'Content-Type: application/json' \
//!   -d '[
//!     {
//!       "name": "Dasher",
//!       "strength": 5,
//!       "speed": 50.4,
//!       "height": 80,
//!       "antler_width": 36,
//!       "snow_magic_power": 9001,
//!       "favorite_food": "hay",
//!       "cAnD13s_3ATeN-yesT3rdAy": 2
//!     },
//!     {
//!       "name": "Dancer",
//!       "strength": 6,
//!       "speed": 48.2,
//!       "height": 65,
//!       "antler_width": 37,
//!       "snow_magic_power": 4004,
//!       "favorite_food": "grass",
//!       "cAnD13s_3ATeN-yesT3rdAy": 5
//!     }
//!   ]'
//! ```
//!
//! ## Example Output
//!
//! ```not_rust
//! {
//!   "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
//!   "tallest": "Dasher is standing tall with his 36 cm wide antlers",
//!   "magician": "Dasher could blast you away with a snow magic power of 9001",
//!   "consumer": "Dancer ate lots of candies, but also some grass"
//! }
//! ```
use axum::{extract, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

/// Get Day 4 routes
///
/// * `/4/strength`
/// * `/4/contest`
pub fn get_routes() -> Router {
    Router::new()
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest))
}

/// Deer data POSTed to the supported routes
#[derive(Deserialize)]
struct Deer {
    name: String,
    strength: i32,
    speed: Option<f32>,
    height: Option<u16>,
    antler_width: Option<u16>,
    snow_magic_power: Option<u16>,
    favorite_food: Option<String>,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten_yesterday: Option<u8>,
}

/// Response data for the contest request
#[derive(Serialize, Default, Debug)]
struct Contest {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

/// Find the strongest deer
async fn strength(extract::Json(payload): extract::Json<Vec<Deer>>) -> String {
    payload
        .iter()
        .fold(0i32, |acc, deer| acc + deer.strength)
        .to_string()
}

/// Generate a `Contest` response
async fn contest(extract::Json(payload): extract::Json<Vec<Deer>>) -> Json<Contest> {
    let mut fastest = 0;
    let mut tallest = 0;
    let mut magician = 0;
    let mut consumer = 0;

    for (idx, deer) in payload.iter().enumerate() {
        if deer.speed.unwrap_or_default() > payload[fastest].speed.unwrap_or_default() {
            fastest = idx;
        }
        if deer.height.unwrap_or_default() > payload[tallest].height.unwrap_or_default() {
            tallest = idx;
        }
        if deer.snow_magic_power.unwrap_or_default()
            > payload[magician].snow_magic_power.unwrap_or_default()
        {
            magician = idx;
        }
        if deer.candies_eaten_yesterday.unwrap_or_default()
            > payload[consumer]
                .candies_eaten_yesterday
                .unwrap_or_default()
        {
            consumer = idx;
        }
    }

    let response = Contest {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            payload[fastest].strength, payload[fastest].name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            payload[tallest].name,
            payload[tallest].antler_width.unwrap_or_default()
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            payload[magician].name,
            payload[magician].snow_magic_power.unwrap_or_default()
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            payload[consumer].name,
            payload[consumer].favorite_food.as_ref().unwrap()
        ),
    };
    Json(response)
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use serde_json::json;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_task1() {
        let app = get_routes();
        let input = json!([
            { "name": "Dasher", "strength": 5 },
            { "name": "Dancer", "strength": 6 },
            { "name": "Prancer", "strength": 4 },
            { "name": "Vixen", "strength": 7 }
        ])
        .to_string();

        let req = Request::builder()
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .uri("/4/strength")
            .body(Body::from(input))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("Failed to convert body to string");

        assert_eq!(body_str, "22");
    }

    #[tokio::test]
    async fn test_task2() {
        let app = get_routes();
        let input = json!([
          {
            "name": "Dasher",
            "strength": 5,
            "speed": 50.4,
            "height": 80,
            "antler_width": 36,
            "snow_magic_power": 9001,
            "favorite_food": "hay",
            "cAnD13s_3ATeN-yesT3rdAy": 2
          },
          {
            "name": "Dancer",
            "strength": 6,
            "speed": 48.2,
            "height": 65,
            "antler_width": 37,
            "snow_magic_power": 4004,
            "favorite_food": "grass",
            "cAnD13s_3ATeN-yesT3rdAy": 5
          }
        ])
        .to_string();

        let req = Request::builder()
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .uri("/4/contest")
            .body(Body::from(input))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_json: serde_json::Value =
            serde_json::from_slice(&body_bytes).expect("Failed to convert body to json");
        let expected_json = json!({
          "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
          "tallest": "Dasher is standing tall with his 36 cm wide antlers",
          "magician": "Dasher could blast you away with a snow magic power of 9001",
          "consumer": "Dancer ate lots of candies, but also some grass"
        });

        assert_eq!(body_json, expected_json);
    }
}
