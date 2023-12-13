use axum::http::StatusCode;

// Day -1
pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

// Day -1
pub async fn error() -> Result<(), StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
