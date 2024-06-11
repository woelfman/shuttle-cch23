//! Day 18: Santa's Gift Orders: Data Analytics Edition
//!
//! Santa sat back in his plush seat, a mug of hot cocoa in his hand, and a
//! smile on his jolly face. The database upgrade from the previous week had
//! indeed worked out exceptionally well; the operations were running smoother
//! than ever, the reports were accurate, and morale among his helpers was at an
//! all-time high. This modern marvel of technology had infused a new spirit
//! into the North Pole operations.
//!
//! # Task 1: Mr. Worldwide
//!
//! This challenge continues from what was built for the Core tasks on Day 13.
//!
//! Santa is stoked about the speed and reliability of the new gift order
//! database backend! He wants you to expand it to support per-region analytics.
//!
//! Copy the `/13/reset` endpoint from Day 13 to `/18/reset`, but modify the
//! query like this:
//!
//! ```not_rust
//! DROP TABLE IF EXISTS regions;
//! DROP TABLE IF EXISTS orders;
//!
//! CREATE TABLE regions (
//!   id INT PRIMARY KEY,
//!   name VARCHAR(50)
//! );
//!
//! CREATE TABLE orders (
//!   id INT PRIMARY KEY,
//!   region_id INT,
//!   gift_name VARCHAR(50),
//!   quantity INT
//! );
//! ```
//!
//! We want to re-use the POST endpoint `/13/orders` at `/18/orders` for adding new
//! orders. You can either add the same handler under the new route, or just
//! copy+paste the entire thing, as long as both endpoints are doing the same
//! thing.
//!
//! Now, add a POST endpoint `/18/regions` that inserts regions in the same way
//! the orders endpoint does.
//!
//! Lastly, add a GET endpoint `/18/regions/total` that returns the total number
//! of orders per region. To make it easier for Santa to find a location, the
//! output should be alphabetically sorted on the region name. Regions with no
//! orders should not be listed in the result.
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/18/reset
//! curl -X POST http://localhost:8000/18/regions \
//! -H 'Content-Type: application/json' \
//! -d '[
//!   {"id":1,"name":"North Pole"},
//!   {"id":2,"name":"Europe"},
//!   {"id":3,"name":"North America"},
//!   {"id":4,"name":"South America"},
//!   {"id":5,"name":"Africa"},
//!   {"id":6,"name":"Asia"},
//!   {"id":7,"name":"Oceania"}
//! ]'
//! curl -X POST http://localhost:8000/18/orders \
//! -H 'Content-Type: application/json' \
//! -d '[
//!   {"id":1,"region_id":2,"gift_name":"Board Game","quantity":5},
//!   {"id":2,"region_id":2,"gift_name":"Origami Set","quantity":8},
//!   {"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
//!   {"id":4,"region_id":4,"gift_name":"Teddy Bear","quantity":10},
//!   {"id":5,"region_id":2,"gift_name":"Yarn Ball","quantity":6},
//!   {"id":6,"region_id":3,"gift_name":"Art Set","quantity":3},
//!   {"id":7,"region_id":5,"gift_name":"Robot Lego Kit","quantity":5},
//!   {"id":8,"region_id":6,"gift_name":"Drone","quantity":9}
//! ]'
//! curl http://localhost:8000/18/regions/total
//!
//! [
//! {"region":"Africa","total":5},
//! {"region":"Asia","total":9},
//! {"region":"Europe","total":19},
//! {"region":"North America","total":15},
//! {"region":"South America","total":10}
//! ]
//! ```
//!
//! # Task 2: West Pole to East Pole - Santa wants ALL the data
//!
//! To optimize production of gifts for next year, Santa needs detailed insights
//! into the best performing gifts in every region.
//!
//! Create a GET endpoint `/18/regions/top_list/<number>` that retrieves the
//! names of the regions along with the top `<number>` most ordered gifts in
//! each region, considering the quantity of orders placed for each gift.
//!
//! If there are less than `<number>` unique gifts in a region, the top list
//! will be shorter. If there are no gifts in a region, show that with an empty
//! top list.
//!
//! If there is a tie among gifts, use alphabetical ordering of the gift name to
//! break it. The final output shall once again be ordered by region name.
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/18/reset
//! curl -X POST http://localhost:8000/18/regions \
//! -H 'Content-Type: application/json' \
//! -d '[
//!   {"id":1,"name":"North Pole"},
//!   {"id":2,"name":"South Pole"},
//!   {"id":3,"name":"Kiribati"},
//!   {"id":4,"name":"Baker Island"}
//! ]'
//! curl -X POST http://localhost:8000/18/orders \
//! -H 'Content-Type: application/json' \
//! -d '[
//!   {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
//!   {"id":2,"region_id":2,"gift_name":"Toy Train","quantity":3},
//!   {"id":3,"region_id":2,"gift_name":"Doll","quantity":8},
//!   {"id":4,"region_id":3,"gift_name":"Toy Train","quantity":3},
//!   {"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
//!   {"id":6,"region_id":3,"gift_name":"Action Figure","quantity":12},
//!   {"id":7,"region_id":4,"gift_name":"Board Game","quantity":10},
//!   {"id":8,"region_id":3,"gift_name":"Teddy Bear","quantity":1},
//!   {"id":9,"region_id":3,"gift_name":"Teddy Bear","quantity":2}
//! ]'
//! curl http://localhost:8000/18/regions/top_list/2
//!
//! [
//! {"region":"Baker Island","top_gifts":["Board Game"]},
//! {"region":"Kiribati","top_gifts":["Action Figure","Teddy Bear"]},
//! {"region":"North Pole","top_gifts":[]},
//! {"region":"South Pole","top_gifts":["Doll","Toy Train"]}
//! ]
//! ```
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use sea_query::{Alias, ColumnDef, Expr, Iden, Order, PostgresQueryBuilder, Query, Table};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

/// Get Day 18 routes
///
/// * `/18/reset`
/// * `/18/orders`
/// * `/18/regions`
/// * `/18/regions/total`
/// * `/18/regions/top_list/<number>`
pub fn get_routes(pool: PgPool) -> Router {
    let state = AppState { pool };

    Router::new()
        .route("/18/reset", post(reset))
        .route("/18/orders", post(orders)) // Reuse d13 orders
        .route("/18/regions", post(regions))
        .route("/18/regions/total", get(regions_total))
        .route("/18/regions/top_list/:number", get(top_list))
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    pub pool: PgPool,
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

#[derive(Deserialize)]
struct OrderStruct {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

async fn reset(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    let query = Table::drop()
        .table(Regions::Table)
        .if_exists()
        .build(PostgresQueryBuilder);

    sqlx::query(&query)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let query = Table::create()
        .table(Regions::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Regions::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Regions::Name).string_len(50))
        .build(PostgresQueryBuilder);

    sqlx::query(&query)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
        .col(
            ColumnDef::new(Orders::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
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

#[derive(Iden)]
enum Regions {
    Id,
    Name,
    Table,
}

#[derive(Iden)]
enum Orders {
    GiftName,
    Id,
    Quantity,
    RegionId,
    Table,
}

#[derive(Deserialize)]
struct Region {
    id: i32,
    name: String,
}

async fn regions(
    State(state): State<AppState>,
    Json(regions): Json<Vec<Region>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    for Region { id, name } in regions {
        let (sql, values) = Query::insert()
            .into_table(Regions::Table)
            .columns([Regions::Id, Regions::Name])
            .values_panic([id.into(), name.into()])
            .returning_col(Regions::Name)
            .build_sqlx(PostgresQueryBuilder);

        let _row = sqlx::query_with(&sql, values)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(StatusCode::OK)
}

#[derive(FromRow, Serialize)]
struct Total {
    #[serde(rename(serialize = "region"))]
    name: String,
    total: Option<i64>,
}

async fn regions_total(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (sql, values) = Query::select()
        .column(Regions::Name)
        .expr_as(
            Expr::col((Orders::Table, Orders::Quantity)).sum(),
            Alias::new("total"),
        )
        .from(Regions::Table)
        .left_join(
            Orders::Table,
            Expr::col((Orders::Table, Orders::RegionId)).equals((Regions::Table, Regions::Id)),
        )
        .group_by_col((Regions::Table, Regions::Name))
        .order_by((Regions::Table, Regions::Name), Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let rows: Vec<Total> = sqlx::query_as_with::<_, Total, _>(&sql, values)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .into_iter()
        .filter(|row| row.total.is_some())
        .collect();

    Ok(Json(rows))
}

#[derive(FromRow, Serialize)]
struct TopGifts {
    region: String,
    top_gifts: Vec<String>,
}

#[derive(FromRow)]
struct RegionStruct {
    id: i32,
    name: String,
}

#[derive(FromRow)]
struct TopGiftsStruct(String);

async fn top_list(
    State(state): State<AppState>,
    Path(number): Path<u64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut top_list: Vec<TopGifts> = Vec::new();

    let (sql, values) = Query::select()
        .column(Regions::Id)
        .column(Regions::Name)
        .from(Regions::Table)
        .order_by(Regions::Name, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let regions = sqlx::query_as_with::<_, RegionStruct, _>(&sql, values)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for region in regions {
        let (sql, values) = Query::select()
            .column(Orders::GiftName)
            .from(Orders::Table)
            .and_where(Expr::col(Orders::RegionId).eq(region.id))
            .limit(number)
            .group_by_col(Orders::GiftName)
            .order_by_expr(Expr::col(Orders::Quantity).sum(), Order::Desc)
            .build_sqlx(PostgresQueryBuilder);

        let top_gifts = sqlx::query_as_with::<_, TopGiftsStruct, _>(&sql, values)
            .fetch_all(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .into_iter()
            .map(|tg| tg.0)
            .collect();

        top_list.push(TopGifts {
            region: region.name,
            top_gifts,
        });
    }

    Ok(Json(top_list))
}
