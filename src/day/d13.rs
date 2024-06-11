//! Day 13: Santa's Gift Orders
//!
//! Santa Claus has started facing a pressing issue at the North Pole. The
//! existing database, written in a legacy language, is becoming insufficient
//! for handling the tidal wave of gift requests from children worldwide. This
//! ancient system is not only slowing down operations, but it is also proving
//! harder to maintain.
//!
//! To ensure that not a single child's wish is overlooked and operations run as
//! efficiently as possible, an immediate upgrade is a necessity.
//!
//! # Task 1: SQL? Sequel? Squeel??
//!
//! Santa's gift order database is written in an ancient language and needs to
//! be oxidized. Let's show him the power of Rust with your backend combined
//! with a Postgres database.
//!
//! Add a Postgres database with the [`Shuttle Shared
//! Database`](https://docs.shuttle.rs/resources/shuttle-shared-db) plugin, and
//! add the pool to your application state. Add a GET endpoint `/13/sql` that
//! executes the SQL query `SELECT 20231213` and responds with the query result
//! (an `i32` turned into a string).
//!
//! ## Example
//!
//! ```not_rust
//! curl http://localhost:8000/13/sql
//!
//! 20231213
//! ```
//!
//! # Task 2: Use code NorthPole2023 for 2023% off???
//!
//! Now that the data can be migrated over to the new database, we see that
//! Santa's workshop has received numerous gift orders from different regions.
//! Time to do some basic analysis.
//!
//! Create a POST endpoint `/13/reset` that (re-)creates the following schema in
//! your database upon being called, and returns a plain `200 OK`. It will be
//! used at the start of each test to ensure a clean starting point.
//!
//! Then, create a POST endpoint `/13/orders` that takes a JSON array of order
//! objects and inserts them into the table (see below). Return a plain `200
//! OK`.
//!
//! Lastly, create a GET endpoint `/13/orders/total` that queries the table and
//! returns the total number of gifts ordered (the sum of all quantities).
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/13/reset
//! curl -X POST http://localhost:8000/13/orders \
//! -H 'Content-Type: application/json' \
//! -d '[
//!   {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
//!   {"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
//!   {"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
//!   {"id":4,"region_id":4,"gift_name":"Board Game","quantity":10},
//!   {"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
//!   {"id":6,"region_id":3,"gift_name":"Toy Train","quantity":3}
//! ]'
//! curl http://localhost:8000/13/orders/total
//!
//! {"total":44}
//! ```
//!
//! # Task 3: Truly one of the gifts of all time
//!
//! Add a GET endpoint `/13/orders/popular` that returns the name of the most
//! popular gift. If there is no most popular gift, use `null` instead of a
//! string.
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/13/reset
//! curl -X POST http://localhost:8000/13/orders \
//! -H 'Content-Type: application/json' \
//! -d '[
//!   {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
//!   {"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
//!   {"id":3,"region_id":3,"gift_name":"Toy Train","quantity":4}
//! ]'
//! curl http://localhost:8000/13/orders/popular
//!
//! {"popular":"Toy Train"}
//! ```
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sea_query::{Alias, ColumnDef, Expr, Iden, Order, PostgresQueryBuilder, Query, Table};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};

/// Get Day 13 routes
///
/// * `/13/sql`
/// * `/13/reset``
/// * `/13/orders`
/// * `/13/orders/total`
/// * `/13/orders/popular`
pub fn get_routes(pool: PgPool) -> Router {
    let state = AppState { pool };

    Router::new()
        .route("/13/sql", get(sql))
        .route("/13/reset", post(reset))
        .route("/13/orders", post(orders))
        .route("/13/orders/total", get(orders_total))
        .route("/13/orders/popular", get(orders_popular))
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(FromRow)]
struct Task1(i32);

async fn sql(State(state): State<AppState>) -> Result<impl IntoResponse, impl IntoResponse> {
    let (sql, values) = Query::select()
        .expr(Expr::val(20231213))
        .build_sqlx(PostgresQueryBuilder);

    match sqlx::query_as_with::<_, Task1, _>(&sql, values)
        .fetch_one(&state.pool)
        .await
    {
        Ok(task) => Ok((StatusCode::OK, task.0.to_string())),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[derive(Iden)]
enum Orders {
    Table,
    Id,
    RegionId,
    GiftName,
    Quantity,
}

async fn reset(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    let query = Table::drop()
        .table(Orders::Table)
        .if_exists()
        .build(PostgresQueryBuilder);

    sqlx::query(&query)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let query = Table::create()
        .table(Orders::Table)
        .if_not_exists()
        .col(ColumnDef::new(Orders::Id).integer().primary_key())
        .col(ColumnDef::new(Orders::RegionId).integer())
        .col(ColumnDef::new(Orders::GiftName).string_len(50))
        .col(ColumnDef::new(Orders::Quantity).integer())
        .build(PostgresQueryBuilder);

    sqlx::query(&query)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
struct OrderStruct {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

async fn orders(
    State(state): State<AppState>,
    Json(orders): Json<Vec<OrderStruct>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    for OrderStruct {
        id,
        region_id,
        gift_name,
        quantity,
    } in orders
    {
        let (sql, values) = Query::insert()
            .into_table(Orders::Table)
            .columns([
                Orders::Id,
                Orders::RegionId,
                Orders::GiftName,
                Orders::Quantity,
            ])
            .values_panic([
                id.into(),
                region_id.into(),
                gift_name.clone().into(),
                quantity.into(),
            ])
            .returning_col(Orders::Id)
            .build_sqlx(PostgresQueryBuilder);

        let _row = sqlx::query_with(&sql, values)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(StatusCode::OK)
}

#[derive(FromRow)]
struct Task2(i64);

async fn orders_total(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (sql, values) = Query::select()
        .expr_as(Expr::col(Orders::Quantity).sum(), Alias::new("i64"))
        .from(Orders::Table)
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_as_with::<_, Task2, _>(&sql, values)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(format!("{{\"total\":{}}}", row.0))
}

#[derive(FromRow)]
struct OrdersPopular {
    sq: i64,
    gift_name: String,
}

async fn orders_popular(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (sql, values) = Query::select()
        .expr_as(Expr::col(Orders::Quantity).sum(), Alias::new("sq"))
        .column(Orders::GiftName)
        .from(Orders::Table)
        .group_by_col(Orders::GiftName)
        .order_by(Alias::new("sq"), Order::Desc)
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, OrdersPopular, _>(&sql, values)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match rows.len() {
        0 => Ok("{\"popular\":null}".to_string()),
        1 => Ok(format!("{{\"popular\":\"{}\"}}", rows[0].gift_name)),
        _ if rows[0].sq == rows[1].sq => Ok("{\"popular\":null}".to_string()),
        _ => Ok(format!("{{\"popular\":\"{}\"}}", rows[0].gift_name)),
    }
}
