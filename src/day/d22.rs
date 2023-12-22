use axum::{routing::post, Router};
use itertools::Itertools;

pub fn get_routes() -> Router {
    Router::new()
        .route("/22/integers", post(integers))
}

async fn integers(body: String) -> String {
    let mut ints: Vec<u64> = body.lines().filter_map(|n| n.parse().ok()).collect();

    ints.sort();

    for mut chunk in &ints.into_iter().chunks(2) {
        let a = chunk.next();
        let b = chunk.next();

        if a != b {
            return '🎁'.to_string().repeat(a.unwrap() as usize);
        }
    }

    "".to_string()
}
