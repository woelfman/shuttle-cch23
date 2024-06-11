//! Day 11: Imagery from the North Pole
//!
//! Decked out in his signature red coat, Santa's eyes sparkle brighter than the
//! Northern Star as he navigates through tall shelves packed with newly
//! produced Christmas decorations for the season. Handcrafted glass balls,
//! ornate stars, whimsical snowflakes, festive tinsel - you name it, they have
//! it all.
//!
//! # Task 1: Served on a silver platter
//!
//! The time has come to decorate our surroundings! The elves are getting tired
//! of working with just strings and numbers and bytes, and are in need of some
//! fancy christmas ornaments on the computer screens.
//!
//! Serve an image as a static file so that a GET request to
//! `/11/assets/decoration.png` responds with the image file and correct headers
//! for MIME type (`Content-Type: image/png`) and response length
//! (`Content-Length: ...`).
//!
//! ## Example Input
//!
//! ```not_rust
//! curl -I -X GET http://localhost:8000/11/assets/decoration.png
//! ```
//!
//! ## Example Output
//!
//! ```not_rust
//! HTTP/1.1 200 OK
//! content-type: image/png
//! content-length: 787297
//! ...
//! ```
//!
//! # Task 2: Bull mode activated
//!
//! Add a POST endpoint `/11/red_pixels`, that takes in a PNG image in the
//! `image` field of a multipart POST request, and returns the number of pixels
//! in the image that are perceived as "magical red" when viewed with Santa's
//! night vision goggles. The goggles considers a pixel "magical red" if the
//! color values of the pixel fulfill the formula `red > blue + green`.
//!
//! ## Example
//!
//! ```no_rust
//! curl -X POST http://localhost:8000/11/red_pixels \
//!   -H 'Content-Type: multipart/form-data' \
//!   -F 'image=@decoration.png' # the image from Task 1
//!
//! 73034
//! ```
use std::io::Cursor;

use axum::{extract::Multipart, http::StatusCode, routing::post, Router};
use image::GenericImageView;
use tower_http::services::ServeDir;

/// Get Day 11 routes
///
/// * `/11/assets`
/// * `/11/red_pixels`
pub fn get_routes() -> Router {
    Router::new()
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", post(red_pixels))
}

async fn red_pixels(mut multipart: Multipart) -> Result<String, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "image" {
            let data = field.bytes().await.unwrap();
            let decoder = image::io::Reader::new(Cursor::new(data))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            let magical_red = decoder.pixels().fold(0u64, |acc, (_x, _y, p)| {
                if u16::from(p[0]) > u16::from(p[1]) + u16::from(p[2]) {
                    acc + 1
                } else {
                    acc
                }
            });

            return Ok(magical_red.to_string());
        }
    }

    Err(StatusCode::BAD_REQUEST)
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use axum_test::TestServer;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_task1() {
        let app = get_routes();

        let req = Request::builder()
            .method(Method::GET)
            .uri("/11/assets/decoration.png")
            .body(Body::from(()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.headers()[header::CONTENT_TYPE], "image/png");
        assert_eq!(response.headers()[header::CONTENT_LENGTH], "787297");
    }

    #[tokio::test]
    async fn test_task2() {
        let app = get_routes();

        let server = TestServer::new(app).unwrap();

        let file = std::fs::read("assets/decoration.png").unwrap();
        let part = axum_test::multipart::Part::bytes(file);
        let form = axum_test::multipart::MultipartForm::new().add_part("image", part);

        let response = server.post("/11/red_pixels").multipart(form).await;

        response.assert_text("73034");
    }
}
