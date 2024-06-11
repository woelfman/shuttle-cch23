//! Day 14: Reindeering HTML
//!
//! Did you hear about the time when Santa became a web designer? He picked up
//! coding with great enthusiasm. Each tag told a story, every element was a
//! toy, and every attribute was a wish from a child around the world. He soon
//! managed to build a website where children could easily send their letters
//! filled with Christmas wishes, and the elves could more efficiently organize
//! the toymaking process.
//!
//! # Task 1: Ho-ho, Toymaking Magic Land! (HTML)
//!
//! Today we are simulating an incident that happened shortly after Santa joined
//! the web dev team at the North Pole.
//!
//! Implement a POST endpoint `/14/unsafe` that takes some HTML content and
//! _unsafely_ renders it on a small HTML page.
//!
//! ## Example Input
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/14/unsafe \
//!   -H "Content-Type: application/json" \
//!   -d '{"content": "<h1>Welcome to the North Pole!</h1>"}'
//! ```
//!
//! ## Example Output
//!
//! ```not_rust
//! <html>
//!   <head>
//!   <title>CCH23 Day 14</title>
//! </head>
//! <body>
//!   <h1>Welcome to the North Pole!</h1>
//! </body>
//! </html>
//! ```
//!
//! # Task 2: Safety 2nd
//!
//! Time to clean up the mess that Santa caused in Task 1. Show him how it's
//! done in `/14/safe` by securely rendering the HTML against script injection.
//!
//! ## Example Input
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/14/safe \
//!   -H "Content-Type: application/json" \
//!   -d '{"content": "<script>alert(\"XSS Attack!\")</script>"}'
//! ```
//!
//! ## Example Output
//!
//! ```not_rust
//! <html>
//!   <head>
//!   <title>CCH23 Day 14</title>
//! </head>
//! <body>
//!   &lt;script&gt;alert(&quot;XSS Attack!&quot;)&lt;/script&gt;
//! </body>
//! </html>
//! ```
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use handlebars::{no_escape, Handlebars};
use serde::{Deserialize, Serialize};

/// Get Day 14 routes
///
/// * `/14/unsafe`
/// * `/14/safe`
pub fn get_routes() -> Router {
    Router::new()
        .route("/14/unsafe", post(r#unsafe))
        .route("/14/safe", post(safe))
}

#[derive(Deserialize, Serialize)]
struct Payload {
    content: String,
}

async fn r#unsafe(Json(payload): Json<Payload>) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(no_escape);
    let source = "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{content}}
  </body>
</html>";
    handlebars
        .register_template_string("t1", source)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok::<(StatusCode, String), (StatusCode, String)>((
        StatusCode::OK,
        handlebars.render("t1", &payload).unwrap(),
    ))
}

async fn safe(Json(payload): Json<Payload>) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut handlebars = Handlebars::new();
    let source = "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{content}}
  </body>
</html>";
    handlebars
        .register_template_string("t1", source)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok::<(StatusCode, String), (StatusCode, String)>((
        StatusCode::OK,
        handlebars.render("t1", &payload).unwrap(),
    ))
}
