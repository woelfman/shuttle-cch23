//! Day 6: Elf on a shelf
//!
//! It's that time of year when the elves hide on shelves to watch over the
//! children of the world, reporting back to Santa on who's been naughty or
//! nice. However, this year's reports have been mixed up with the rest of the
//! letters to Santa, and the word "elf" is hidden throughout a mountain of
//! text.
//!
//! # Task 1: Never count on an elf
//!
//! Elves are notorious for their hide-and-seek skills, and now they've hidden
//! themselves in strings of text!
//!
//! Create an endpoint `/6` that takes a POST request with a raw string as input
//! and count how many times the substring `"elf"` appears.
//!
//! The output should be a JSON object containing the count of the string `"elf"`.
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/6 \
//!   -H 'Content-Type: text/plain' \
//!   -d 'The mischievous elf peeked out from behind the toy workshop,
//!       and another elf joined in the festive dance.
//!       Look, there is also an elf on that shelf!'
//!
//! {"elf":4}
//! ```
//!
//! # Task 2: Shelf under an elf?
//!
//! Add two fields to the response that counts:
//!
//! * The number of occurrences of the string `"elf on a shelf"` in a field with
//!   the same name.
//! * number of shelves that don't have an elf on it. That is,
//!   the number of strings `"shelf"` that are not preceded by the string `"elf on a "`.
//!   Put this count in the field `"shelf with no elf on it"`.
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/6 \
//!   -H 'Content-Type: text/plain' \
//!   -d 'there is an elf on a shelf on an elf.
//!       there is also another shelf in Belfast.'
//!
//! {"elf":5,"elf on a shelf":1,"shelf with no elf on it":1}
//! ```
use axum::{routing::post, Json, Router};
use serde::Serialize;

/// Get Day 6 routes
///
/// * `/6`
pub fn get_routes() -> Router {
    Router::new().route("/6", post(elf))
}

#[derive(Default, Serialize, Debug)]
struct Elf {
    elf: u32,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: u32,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf: u32,
}

async fn elf(payload: String) -> Json<Elf> {
    const EOAS: &str = "elf on a shelf";
    let mut elves = Elf::default();
    let mut idx = 0;

    while let Some(i) = payload[idx..].find("elf") {
        elves.elf += 1;
        if payload[(idx + i)..].starts_with(EOAS) {
            elves.elf_on_a_shelf += 1;
        }
        idx += i + "elf".len();
    }

    idx = 0;
    while let Some(i) = payload[idx..].find("shelf") {
        idx += i + "shelf".len();
        if !payload[0..idx].ends_with(EOAS) {
            elves.shelf_with_no_elf += 1;
        }
    }

    Json(elves)
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use serde_json::{json, Value};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_task1() {
        let app = get_routes();

        let input = "The mischievous elf peeked out from behind the toy workshop,
            and another elf joined in the festive dance.
            Look, there is also an elf on that shelf!";

        let req = Request::builder()
            .method(Method::POST)
            .header("Content-Type", "text/plain")
            .uri("/6")
            .body(Body::from(input))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_json: Value =
            serde_json::from_slice(&body_bytes).expect("Failed to convert body to json");

        assert_eq!(body_json["elf"], 4);
    }

    #[tokio::test]
    async fn test_task2() {
        let app = get_routes();

        let input = "there is an elf on a shelf on an elf.
            there is also another shelf in Belfast.";

        let req = Request::builder()
            .method(Method::POST)
            .header("Content-Type", "text/plain")
            .uri("/6")
            .body(Body::from(input))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_json: Value =
            serde_json::from_slice(&body_bytes).expect("Failed to convert body to json");
        let expected_json = json!({"elf": 5, "elf on a shelf": 1, "shelf with no elf on it": 1});

        assert_eq!(body_json, expected_json);
    }
}
