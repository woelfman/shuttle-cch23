//! Day -1: Get your winter boots on!
//!
//! This challenge is a warmup challenge made to familiarize you with deploying
//! your CCH23 project on Shuttle, and it does not count towards your score.
//!
//! # Task 1: Everything is OK
//!
//! Deploy a minimal working web app to your CCH23 Shuttle project.
//!
//! The root endpoint `/` should respond with a `200 OK` status code to GET
//! requests. Responding with a "Hello, world!" string, like the starter
//! templates do, is enough to accomplish this.
//!
//! ## Example Input
//!
//! ```not_rust
//! # On a local run with `cargo shuttle run`
//! curl -I -X GET http://localhost:8000/
//! ```
//!
//! ## Example Output
//!
//! ```not_rust
//! HTTP/1.1 200 OK
//! ...
//! ```
//! # Task 2: Fake error
//!
//! For this bonus task, add an endpoint on `/-1/error` that responds with the
//! status code `500 Internal Server Error` to GET requests. The response body
//! content does not matter.
use axum::{routing::get, Router};

/// Get Day -1 routes
///
/// * `/`
/// * `/-1/error`
pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(task1::hello_world))
        .route("/-1/error", get(task2::error))
}

mod task1 {
    pub async fn hello_world() -> &'static str {
        "Hello, world!"
    }
}

mod task2 {
    use axum::http::StatusCode;

    pub async fn error() -> Result<(), StatusCode> {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_task1() {
        let app = get_routes();

        let req = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::from(()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_str =
            String::from_utf8(body_bytes.to_vec()).expect("Failed to convert body to string");

        assert_eq!(body_str, "Hello, world!");
    }

    #[tokio::test]
    async fn test_task2() {
        let app = get_routes();

        let req = Request::builder()
            .method(Method::GET)
            .uri("/-1/error")
            .body(Body::from(()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
