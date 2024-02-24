use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use sea_query::{Alias, ColumnDef, Expr, Iden, Order, PostgresQueryBuilder, Query, Table};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::FromRow;

use crate::AppState;

#[derive(FromRow)]
struct Task1(i32);

pub async fn sql(State(state): State<AppState>) -> Result<impl IntoResponse, impl IntoResponse> {
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

pub async fn reset(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
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
pub struct OrderStruct {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

pub async fn orders(
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

pub async fn orders_total(
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

pub async fn orders_popular(
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
