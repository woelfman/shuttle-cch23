//! Day 7: GET Santa some cookies
//!
//! At Santa's base near the North Pole (64 km away to be precise), the scent of
//! freshly baked cookies fills the air, a sign that Christmas is near. Santa,
//! however, has forgotten the encoding method that was used to hide his
//! favorite cookie recipe in web browsers around the world. He needs this
//! super-secret recipe (based on his grandfather's recipe made in 1964!) to
//! fuel his late-night toy-making sessions.
//!
//! # Task 1: Based encoding, 64th edition
//!
//! Santa's secret cookie recipe is hidden in an encoded message, and he's
//! looking to you for help. He's sending a GET request to your server with a
//! `Cookie` header.
//!
//! What you need to do is parse the Cookie header, decode the value in the
//! recipe field, and return it.
//!
//! Make an endpoint `/7/decode` that extracts the `Cookie` HTTP header. The header
//! in the request will look something like this:
//!
//! ```not_rust
//! Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==
//! ```
//!
//! After decoding the recipe value to bytes, convert it to a string and return
//! it as the response to the GET request.
//!
//! ## Example
//!
//! ```not_rust
//! curl http://localhost:8000/7/decode \
//!   -H 'Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ=='
//!
//! {"flour":100,"chocolate chips":20}
//! ```
//!
//! # Task 2: The secret cookie recipe
//!
//! Now that the recipe is decoded, Santa can get back to baking cookies! Santa
//! is not sure, however, if he has enough of each ingredient to bake a cookie
//! for every elf.
//!
//! The same type of request as in Task 1 will be sent to a new endpoint,
//! `/7/bake`, but this time Santa needs your help to calculate the amount of
//! cookies he can bake with the ingredients he has in the pantry.
//!
//! After decoding, parse the bytes as a JSON object (shape and keys can be seen
//! in the example below) and use that to calculate how many cookies can be
//! baked with the provided recipe and ingredients. Additionally, return the
//! amount of ingredients that would remain in the pantry after the cookies have
//! been baked.
//!
//! ## Example Input
//!
//! ```not_rust
//! curl http://localhost:8000/7/bake \
//!  -H 'Cookie: recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319'
//! ```
//!
//! After decoding, the recipe above will look like the JSON object:
//!
//! ```not_rust
//! {
//!   "recipe": {
//!     "flour": 95,
//!     "sugar": 50,
//!     "butter": 30,
//!     "baking powder": 10,
//!     "chocolate chips": 50
//!   },
//!   "pantry": {
//!     "flour": 385,
//!     "sugar": 507,
//!     "butter": 2122,
//!     "baking powder": 865,
//!     "chocolate chips": 457
//!   }
//! }
//! ```
//!
//! ## Example Output
//!
//! ```not_rust
//! {
//!   "cookies": 4,
//!   "pantry": {
//!     "flour": 5,
//!     "sugar": 307,
//!     "butter": 2002,
//!     "baking powder": 825,
//!     "chocolate chips": 257
//!   }
//! }
//! ```
//!
//! Explation: The recipe represents the required ingredients to make one
//! cookie. After baking 4 cookies, we have 5 units of flour left and can't bake
//! any more.
//!
//! # Task 3: Questionable cookie recipes
//!
//! Some mischievous elves have now found your endpoint, and are trying their
//! "innovative" cookie recipes on it, without even knowing what ingredients are
//! available in the pantry!
//!
//! Update the endpoint from Task 2 so that any set of ingredients can be
//! present in the recipe and the pantry, respectively.
//!
//! The number of cookies in the answer will always be finite.
//!
//! ## Example
//!
//! ```not_rust
//! curl http://localhost:8000/7/bake \
//!   -H 'Cookie: recipe=eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ=='
//!
//! {
//!   "cookies": 0,
//!   "pantry": {
//!     "cobblestone": 64,
//!     "stick": 4
//!   }
//! }
//! ```
use axum::{http::StatusCode, routing::get, Router};
use axum_extra::extract::CookieJar;
use base64::engine::{general_purpose, Engine};
use serde_json::Value;

/// Get Day 7 routes
///
/// * `/7/decode`
/// * `/7/bake`
pub fn get_routes() -> Router {
    Router::new()
        .route("/7/decode", get(decode))
        .route("/7/bake", get(bake))
}

async fn decode(jar: CookieJar) -> Result<String, StatusCode> {
    if let Some(recipe) = jar.get("recipe") {
        general_purpose::URL_SAFE
            .decode(recipe.value())
            .ok()
            .and_then(|r| String::from_utf8(r).ok())
            .ok_or(StatusCode::NOT_ACCEPTABLE)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn bake(jar: CookieJar) -> Result<String, StatusCode> {
    if let Some(recipe) = jar.get("recipe") {
        let recipe_d = general_purpose::URL_SAFE
            .decode(recipe.value())
            .ok()
            .and_then(|r| String::from_utf8(r).ok())
            .ok_or(StatusCode::NOT_ACCEPTABLE)?;
        let json: Value =
            serde_json::from_str(&recipe_d).map_err(|_| StatusCode::NOT_ACCEPTABLE)?;
        let recipe = json
            .get("recipe")
            .ok_or(StatusCode::NOT_ACCEPTABLE)?
            .clone();
        let mut pantry = json
            .get("pantry")
            .ok_or(StatusCode::NOT_ACCEPTABLE)?
            .clone();

        let mut cookies = u64::MAX;

        // Get the number of cookies we can bake
        for (key, value) in recipe.as_object().unwrap() {
            if let Some(ingredient) = pantry.get(key) {
                if value.as_u64().unwrap_or_default() > 0 {
                    cookies = (ingredient.as_u64().unwrap_or_default()
                        / value.as_u64().unwrap_or(1))
                    .min(cookies);
                }
            }
        }

        // Subtract the ingredients used to make the cookies
        for (key, value) in recipe.as_object().unwrap() {
            if let Some(ingredient) = pantry.get_mut(key) {
                *ingredient = Value::Number(serde_json::Number::from(
                    ingredient.as_u64().unwrap() - cookies * value.as_u64().unwrap(),
                ));
            }
        }

        if cookies == u64::MAX {
            cookies = 0;
        }

        let result = serde_json::json!({
            "cookies": cookies,
            "pantry": pantry,
        })
        .to_string();

        Ok(result)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
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

        let req = Request::builder()
            .method(Method::GET)
            .header(
                "Cookie",
                "recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==",
            )
            .uri("/7/decode")
            .body(Body::from(()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_json: Value =
            serde_json::from_slice(&body_bytes).expect("Failed to convert body to json");

        let expected_json = json!({"flour":100,"chocolate chips":20});

        assert_eq!(body_json, expected_json);
    }

    #[tokio::test]
    async fn test_task2() {
        let app = get_routes();

        let req = Request::builder()
            .method(Method::GET)
            .header("Cookie", "recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319")
            .uri("/7/bake")
            .body(Body::from(()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_json: Value =
            serde_json::from_slice(&body_bytes).expect("Failed to convert body to json");

        let expected_json = json!({
          "cookies": 4,
          "pantry": {
            "flour": 5,
            "sugar": 307,
            "butter": 2002,
            "baking powder": 825,
            "chocolate chips": 257
          }
        });

        assert_eq!(body_json, expected_json);
    }

    #[tokio::test]
    async fn test_task3() {
        let app = get_routes();

        let req = Request::builder()
            .method(Method::GET)
            .header("Cookie", "recipe=eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ==")
            .uri("/7/bake")
            .body(Body::from(()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_json: Value =
            serde_json::from_slice(&body_bytes).expect("Failed to convert body to json");

        let expected_json = json!({
          "cookies": 0,
          "pantry": {
            "cobblestone": 64,
            "stick": 4
          }
        });

        assert_eq!(body_json, expected_json);
    }
}
