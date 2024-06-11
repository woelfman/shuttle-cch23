//! Day 8: PokéPhysics
//!
//! In the heart of the North Pole, Santa's workshop buzzes with a new type of
//! magic. A portal has opened, and Pokémon from another world have tumbled into
//! the snow-dusted realm of elves and reindeer. Santa, ever the innovator, sees
//! an opportunity: why not enlist these charming creatures in his annual
//! gift-giving campaign?
//!
//! But before the sleigh bells ring and the Pokémon can join the flight, Santa
//! needs to understand their unique properties and figure out how many can fit
//! into his sleigh, given their weight.
//!
//! # Task 1: IT'S PIKACHU!
//!
//! Your quest is to add a GET endpoint `/8/weight/<pokedex_number>` that, given a
//! pokédex number, responds with the corresponding Pokémon's weight in
//! kilograms as a number in plain text.
//!
//! ## Example
//!
//! ```not_rust
//! curl http://localhost:8000/8/weight/25
//!
//! 6
//! ```
//!
//! # Task 2: That's gonna leave a dent
//!
//! Once the Pokémon's weight is retrieved, Santa needs you to calculate the
//! momentum it would have at the time of impact with the floor if dropped from
//! a 10-meter high chimney (so that he knows if he needs to climb down or if he
//! can just drop it).
//!
//! Keep in mind that the gravity of Earth that Santa has in his physics book
//! was measured close to the North Pole. This could explain why his
//! calculations are a bit off sometimes, but he still wants you to use it.
//!
//! The momentum, measured in Newton-seconds, signifies the force the Pokémon
//! would exert upon meeting the floor beneath the 10-meter high chimney.
//!
//! The GET endpoint `/8/drop/<pokedex_number>` shall respond with a plain text
//! floating point number.
//!
//! Use gravitational acceleration `g = 9.825 m/s²`. Ignore air resistance.
//!
//! ## Example
//!
//! ```not_rust
//! curl http://localhost:8000/8/drop/25
//!
//! 84.10707461325713
//! ```
use axum::{extract::Path, http::StatusCode, routing::get, Router};
use serde::Deserialize;

/// Get Day 8 routes
///
/// * `/8/weight/<pokedex_number>`
/// * `/8/drop/<pokedex_number>`
pub fn get_routes() -> Router {
    Router::new()
        .route("/8/weight/:pokedex", get(pokedex))
        .route("/8/drop/:pokedex", get(drop))
}

#[derive(Deserialize)]
struct Pokemon {
    weight: u32,
}

async fn pokedex(Path(pokedex): Path<u32>) -> Result<String, StatusCode> {
    let uri = format!("https://pokeapi.co/api/v2/pokemon/{pokedex}");
    let pokemon = reqwest::get(&uri)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .json::<Pokemon>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((f64::from(pokemon.weight) / 10.).to_string())
}

async fn drop(Path(pokedex): Path<u32>) -> Result<String, StatusCode> {
    let uri = format!("https://pokeapi.co/api/v2/pokemon/{pokedex}");
    let pokemon = reqwest::get(&uri)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .json::<Pokemon>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let speed = (2.0f64 * 9.825 * 10.).sqrt();
    let momentum = speed * (f64::from(pokemon.weight) / 10.);
    Ok(momentum.to_string())
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
    async fn test_task1() {
        let app = get_routes();

        let req = Request::builder()
            .method(Method::GET)
            .uri("/8/weight/25")
            .body(Body::from(()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_string =
            String::from_utf8(body_bytes.to_vec()).expect("Failed to convert body to string");

        assert_eq!(body_string, "6");
    }

    #[tokio::test]
    async fn test_task2() {
        let app = get_routes();

        let req = Request::builder()
            .method(Method::GET)
            .uri("/8/drop/25")
            .body(Body::from(()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_string =
            String::from_utf8(body_bytes.to_vec()).expect("Failed to convert body to string");
        let body_f64: f64 = body_string.parse().expect("Failed to convert body to f64");

        assert!((body_f64 - 84.10707461325713).abs() < 0.001)
    }
}
