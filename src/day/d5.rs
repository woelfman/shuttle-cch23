use axum::{extract::Query, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn get_routes() -> Router {
    Router::new().route("/5", post(five))
}

#[derive(Deserialize)]
struct Parms {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn five(parms: Query<Parms>, Json(names): Json<Vec<String>>) -> Json<Value> {
    let mut new_names = Vec::new();
    let offset = parms.offset.unwrap_or(0);
    let limit = parms.limit.unwrap_or(usize::MAX);
    let split = parms.split.unwrap_or(usize::MAX);

    let mut iter = names.iter();
    for _ in 0..offset {
        iter.next();
    }

    for (i, s) in iter.take(limit).enumerate() {
        if i % split == 0 {
            new_names.push(Vec::new());
        }
        new_names.last_mut().unwrap().push(s.clone());
    }

    if parms.split.is_some() {
        Json(json!(new_names))
    } else if let Some(v) = new_names.first() {
        Json(json!(v))
    } else {
        Json(json!([]))
    }
}
