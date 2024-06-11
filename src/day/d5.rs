//! Day 5: Why did Santa's URL query go haywire on Christmas? Too many "present" parameters!
//!
//! In the technologically advanced North Pole, Santa decided to streamline his
//! gift-tracking system using URL query parameters, entrusting the elves with
//! entering present requests. However, the mischievous Grinch added duplicate
//! parameters like "present=puzzle" and "present=unicorn" as a prank. On
//! Christmas Eve, as Santa set out to deliver gifts, the excess parameters
//! caused a glitch: the list of names entered an infinite loop.
//!
//! # Task 1: Slicing the Loop
//!
//! Santa has some lists of names that are becoming too long to deal with. Help
//! him by adding URL query parameters for paginating the list.
//!
//! The task is to create a POST endpoint `/5` that takes a JSON list of names,
//! and query parameters `offset` and `limit` as numbers. Then, return the sub-slice
//! of the list between index `offset` and `offset + limit`.
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST "http://localhost:8000/5?offset=3&limit=5" \
//!   -H 'Content-Type: application/json' \
//!   -d '[
//!     "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe",
//!     "Nolan", "Harper", "Lucas", "Stella", "Mason", "Olivia"
//!   ]'
//!
//! ["Owen", "Lily", "Ethan", "Zoe", "Nolan"]
//! ```
//!
//! # Task 2: Time to Page Some Names
//!
//! This time, Santa also needs to be able to get all pages at once.
//!
//! Modify the same endpoint, so that it can also handle a `split` parameter. All
//! parameters should now be optional. If not given, `offset` defaults to 0, and
//! `limit` defaults to including all remaining items in the list. If `split` is not
//! given, no splitting will happen, but if given, the output list should be
//! split into sub-lists with length according the the value.
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/5?split=4 \
//!   -H 'Content-Type: application/json' \
//!   -d '[
//!     "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe",
//!     "Nolan", "Harper", "Lucas", "Stella", "Mason", "Olivia"
//!   ]'
//!
//! [
//!   ["Ava", "Caleb", "Mia", "Owen"],
//!   ["Lily", "Ethan", "Zoe", "Nolan"],
//!   ["Harper", "Lucas", "Stella", "Mason"],
//!   ["Olivia"]
//! ]
//! ```
//!
//! ```not_rust
//! curl -X POST "http://localhost:8000/5?offset=5&split=2" \
//!   -H 'Content-Type: application/json' \
//!   -d '[
//!     "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe",
//!     "Nolan", "Harper", "Lucas", "Stella", "Mason", "Olivia"
//!   ]'
//!
//! [
//!   ["Ethan", "Zoe"],
//!   ["Nolan", "Harper"],
//!   ["Lucas", "Stella"],
//!   ["Mason", "Olivia"]
//! ]
//! ```
use axum::{extract::Query, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

/// Get Day 5 routes
///
/// * `/5`
pub fn get_routes() -> Router {
    Router::new().route("/5", post(five))
}

#[derive(Deserialize)]
struct Parms {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn five(parms: Query<Parms>, Json(names): Json<Vec<String>>) -> Json<Value> {
    let mut new_names = Vec::new();
    let offset = parms.offset.unwrap_or(0);
    let limit = parms.limit.unwrap_or(usize::MAX);
    let split = parms.split.unwrap_or(usize::MAX);

    let mut iter = names.iter();
    for _ in 0..offset {
        iter.next();
    }

    for (i, s) in iter.take(limit).enumerate() {
        if i % split == 0 {
            new_names.push(Vec::new());
        }
        new_names.last_mut().unwrap().push(s.clone());
    }

    if parms.split.is_some() {
        Json(json!(new_names))
    } else if let Some(v) = new_names.first() {
        Json(json!(v))
    } else {
        Json(json!([]))
    }
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
            "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe", "Nolan", "Harper", "Lucas",
            "Stella", "Mason", "Olivia"
        ])
        .to_string();

        let req = Request::builder()
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .uri("/5?offset=3&limit=5")
            .body(Body::from(input))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_json: Value =
            serde_json::from_slice(&body_bytes).expect("Failed to convert body to json");
        let expected_json = json!(["Owen", "Lily", "Ethan", "Zoe", "Nolan"]);

        assert_eq!(body_json, expected_json);
    }

    #[tokio::test]
    async fn test_task2() {
        let app = get_routes();

        let io = [
            (
                "/5?split=4",
                json!([
                    "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe", "Nolan", "Harper",
                    "Lucas", "Stella", "Mason", "Olivia"
                ])
                .to_string(),
                json!([
                    ["Ava", "Caleb", "Mia", "Owen"],
                    ["Lily", "Ethan", "Zoe", "Nolan"],
                    ["Harper", "Lucas", "Stella", "Mason"],
                    ["Olivia"]
                ]),
            ),
            (
                "/5?offset=5&split=2",
                json!([
                    "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe", "Nolan", "Harper",
                    "Lucas", "Stella", "Mason", "Olivia"
                ])
                .to_string(),
                json!([
                    ["Ethan", "Zoe"],
                    ["Nolan", "Harper"],
                    ["Lucas", "Stella"],
                    ["Mason", "Olivia"]
                ]),
            ),
        ];

        for (uri, body, expected_json) in io {
            let req = Request::builder()
                .method(Method::POST)
                .header("Content-Type", "application/json")
                .uri(uri)
                .body(Body::from(body))
                .unwrap();

            let response = app.clone().oneshot(req).await.unwrap();

            let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("Failed to read response body");

            let body_json: Value =
                serde_json::from_slice(&body_bytes).expect("Failed to convert body to json");

            assert_eq!(body_json, expected_json);
        }
    }
}
