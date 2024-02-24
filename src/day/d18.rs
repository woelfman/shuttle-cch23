use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use sea_query::{Alias, ColumnDef, Expr, Iden, Order, PostgresQueryBuilder, Query, Table};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

pub async fn reset(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
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
pub struct Region {
    id: i32,
    name: String,
}

pub async fn regions(
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

pub async fn regions_total(
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

pub async fn top_list(
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
