use axum::Router;
use cch23_woelfman::day;
use shuttle_runtime::CustomError;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let router = Router::new()
        .merge(day::d_1::get_routes())
        .merge(day::d1::get_routes())
        .merge(day::d4::get_routes())
        .merge(day::d5::get_routes())
        .merge(day::d6::get_routes())
        .merge(day::d7::get_routes())
        .merge(day::d8::get_routes())
        .merge(day::d11::get_routes())
        .merge(day::d12::get_routes())
        .merge(day::d13::get_routes(pool.clone()))
        .merge(day::d14::get_routes())
        .merge(day::d15::get_routes())
        .merge(day::d18::get_routes(pool.clone()))
        .merge(day::d19::get_routes())
        .merge(day::d20::get_routes())
        .merge(day::d21::get_routes())
        .merge(day::d22::get_routes());

    Ok(router.into())
}
