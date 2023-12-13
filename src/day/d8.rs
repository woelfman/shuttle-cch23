use axum::{extract::Path, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pokemon {
    weight: u32,
}

pub async fn pokedex(Path(pokedex): Path<u32>) -> Result<String, StatusCode> {
    let uri = format!("https://pokeapi.co/api/v2/pokemon/{pokedex}");
    let pokemon = reqwest::get(&uri)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .json::<Pokemon>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((pokemon.weight as f64 / 10.).to_string())
}

pub async fn drop(Path(pokedex): Path<u32>) -> Result<String, StatusCode> {
    let uri = format!("https://pokeapi.co/api/v2/pokemon/{pokedex}");
    let pokemon = reqwest::get(&uri)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .json::<Pokemon>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let speed = (2.0f64 * 9.825 * 10.).sqrt();
    let momentum = speed * (pokemon.weight as f64 / 10.);
    Ok(momentum.to_string())
}
