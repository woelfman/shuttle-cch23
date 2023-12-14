use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

#[derive(Serialize, FromRow)]
struct Task1 {
    pub id: i32,
    pub num: i32,
}

pub async fn sql(State(state): State<AppState>) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query!("SELECT 20231213 as \"i32\"")
        .fetch_one(&state.pool)
        .await
    {
        Ok(record) => Ok((StatusCode::OK, record.i32.unwrap().to_string())),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn reset(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query!(
        "CREATE TABLE orders (
        id INT PRIMARY KEY,
        region_id INT,
        gift_name VARCHAR(50),
        quantity INT
    )"
    )
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

pub async fn orders(
    State(state): State<AppState>,
    Json(orders): Json<Vec<Order>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    for Order {
        id,
        region_id,
        gift_name,
        quantity,
    } in orders
    {
        sqlx::query!(
            "INSERT INTO orders VALUES ($1, $2, $3, $4)",
            id,
            region_id,
            gift_name,
            quantity
        )
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(StatusCode::OK)
}

pub async fn orders_total(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let total = sqlx::query!("select sum(quantity) as \"i64!\" from orders")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .i64;

    Ok(format!("{{\"total\":{total}}}"))
}

pub async fn orders_popular(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let gifts = sqlx::query!(
        "select sum(quantity) as \"sq!\", gift_name as \"gift_name!\"
        from orders group by gift_name order by \"sq!\" desc"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match gifts.len() {
        0 => Ok("{\"popular\":null}".to_string()),
        1 => Ok(format!("{{\"popular\":\"{}\"}}", gifts[0].gift_name)),
        _ if gifts[0].sq == gifts[1].sq => Ok("{\"popular\":null}".to_string()),
        _ => Ok(format!("{{\"popular\":\"{}\"}}", gifts[0].gift_name)),
    }
}
