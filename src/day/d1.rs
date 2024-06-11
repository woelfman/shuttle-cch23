//! Day 1: Packet "exclusive-cube" not found
//!
//! In the frosty expanses of the North Pole, Santa's advanced packet management
//! system has encountered a glitch. This system, known for its robustness and
//! magical speed, is responsible for sorting and dispatching all the Christmas
//! gifts. However, a sudden aurora borealis storm has scattered bits of data
//! across the snowy landscape, leaving Santa in dire need of a digital handyman
//! to restore order.
//!
//! # Task 1: Cube the bits
//!
//! Santa needs your programming expertise to recalibrate the packet IDs in his
//! packet management system.
//!
//! Implement a `GET` endpoint `/1/<num1>/<num2>` that takes 2 integers in the
//! path, `num1` and `num2`, and returns the result of `(num1 XOR num2) POW 3`,
//! where *XOR* is the exclusive *OR* operation, and *POW* is exponentiation.
//!
//! ## Example
//!
//! ```not_rust
//! curl http://localhost:8000/1/4/8
//!
//! 1728
//! ```
//!
//! # Task 2: The sled ID system
//!
//! After the packet IDs are calibrated and the packets are packed into sleds,
//! Santa needs to calibrate the sled ID.
//!
//! The formula is very similar: All packet IDs (integers) are *XOR*'ed with
//! each other, and then the result is (again) raised to the power of 3. The
//! catch is that there can be between 1 and 20 packets in a sled!
//!
//! Adapt the endpoint from Task 1 so that it can also be used to calculate sled IDs.
//!
//! ## Examples
//!
//! ```not_rust
//! curl http://localhost:8000/1/10
//!
//! 1000
//! ```
//!
//! ```not_rust
//! curl http://localhost:8000/1/4/5/8/10
//!
//! 27
//! ```
use axum::{extract::Path, http::StatusCode, routing::get, Router};

/// Get Day 1 routes
///
/// * `/1/<num1>/<num2>`
pub fn get_routes() -> Router {
    Router::new().route("/1/*num", get(num))
}

async fn num(Path(num): Path<String>) -> Result<String, StatusCode> {
    let num: Result<Vec<i64>, _> = num.split('/').map(str::parse).collect();
    let num = num.map_err(|_| StatusCode::BAD_REQUEST)?;

    let result: i64 = num.iter().fold(0, |acc, &x| acc ^ x).pow(3);
    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test() {
        let app = get_routes();

        let io = [("/1/4/8", "1728"), ("/1/10", "1000"), ("/1/4/5/8/10", "27")];

        for (uri, expected_body) in io {
            let req = Request::builder()
                .method(Method::GET)
                .uri(uri)
                .body(Body::from(()))
                .unwrap();

            let response = app.clone().oneshot(req).await.unwrap();

            let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("Failed to read response body");

            let body_str =
                String::from_utf8(body_bytes.to_vec()).expect("Failed to convert body to string");

            assert_eq!(body_str, expected_body);
        }
    }
}
