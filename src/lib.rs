use std::collections::HashMap;

use tokio::time::Instant;

pub mod day;

#[derive(Default)]
pub struct AppState {
    pub save_string: HashMap<String, Instant>,
}
