use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use cch23_woelfman::{day, AppState};
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let shared_state = Arc::new(Mutex::new(AppState::default()));

    let router = Router::new()
        .route("/", get(day::d_1::hello_world))
        .route("/-1/error", get(day::d_1::error))
        .route("/1/*num", get(day::d1::num))
        .route("/4/strength", post(day::d4::strength))
        .route("/4/contest", post(day::d4::contest))
        .route("/6", post(day::d6::elf))
        .route("/7/decode", get(day::d7::decode))
        .route("/7/bake", get(day::d7::bake))
        .route("/8/weight/:pokedex", get(day::d8::pokedex))
        .route("/8/drop/:pokedex", get(day::d8::drop))
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", post(day::d11::red_pixels))
        .route("/12/save/:string", post(day::d12::save_string))
        .route("/12/load/:string", get(day::d12::load_string))
        .route("/12/ulids", post(day::d12::ulids))
        .route("/12/ulids/:weekday", post(day::d12::ulids_weekday))
        .with_state(shared_state);

    Ok(router.into())
}
