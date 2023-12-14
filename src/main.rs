use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Router,
};
use cch23_woelfman::{day, AppState};
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let state = AppState {
        save_string: Arc::new(Mutex::new(HashMap::new())),
        pool,
    };

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
        .route("/13/sql", get(day::d13::sql))
        .route("/13/reset", post(day::d13::reset))
        .route("/13/orders", post(day::d13::orders))
        .route("/13/orders/total", get(day::d13::orders_total))
        .route("/13/orders/popular", get(day::d13::orders_popular))
        .route("/14/unsafe", post(day::d14::r#unsafe))
        .route("/14/safe", post(day::d14::safe))
        .with_state(state);

    Ok(router.into())
}
