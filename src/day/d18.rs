use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};

use crate::AppState;

pub async fn reset(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    sqlx::query!("DROP TABLE IF EXISTS regions")
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query!(
        "CREATE TABLE regions (
        id INT PRIMARY KEY,
        name VARCHAR(50)
    )"
    )
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
pub struct Region {
    id: i32,
    name: String,
}

pub async fn regions(
    State(state): State<AppState>,
    Json(regions): Json<Vec<Region>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    for Region { id, name } in regions {
        sqlx::query!("INSERT INTO regions VALUES ($1, $2)", id, name)
            .execute(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(StatusCode::OK)
}

#[derive(Serialize)]
struct Total {
    region: String,
    total: i64,
}

pub async fn regions_total(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut totals = Vec::new();
    let regions = sqlx::query!(
        "SELECT regions.name, SUM(orders.quantity) AS total
        FROM regions
        LEFT JOIN orders ON orders.region_id = regions.id
        GROUP BY regions.name
        ORDER BY regions.name;"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for record in regions.iter().filter(|r| r.total.is_some()) {
        if let (Some(region), Some(total)) = (&record.name, record.total) {
            totals.push(Total {
                region: region.clone(),
                total,
            });
        }
    }

    Ok(Json(totals))
}

#[derive(Serialize)]
struct TopGifts {
    region: String,
    top_gifts: Vec<String>,
}

pub async fn top_list(
    State(state): State<AppState>,
    Path(number): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut top_list = Vec::new();

    let regions = sqlx::query!("SELECT id, name FROM regions ORDER BY name")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for region in regions {
        let top_gifts = sqlx::query!(
            "SELECT gift_name FROM orders
            WHERE region_id = $1
            GROUP BY gift_name
            ORDER BY SUM(quantity)
            DESC LIMIT $2",
            region.id, number
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .into_iter()
        .filter_map(|r| r.gift_name)
        .collect();

        top_list.push(TopGifts {
            region: region.name.unwrap(),
            top_gifts,
        });
    }

    Ok(Json(top_list))
}
