use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use sqlx::PgPool;
use tokio::time::Instant;

pub mod day;

#[derive(Clone)]
pub struct AppState {
    pub save_string: Arc<Mutex<HashMap<String, Instant>>>,
    pub pool: PgPool,
}
