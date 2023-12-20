use std::io::Cursor;

use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use tar::Archive;

pub fn get_routes() -> Router {
    Router::new()
        .route("/20/archive_files", post(archive_files))
        .route("/20/archive_files_size", post(archive_files_size))
}

async fn archive_files(
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Some(ct) = headers.get("Content-Type") {
        if ct != "application/x-tar" {
            return Err((
                StatusCode::BAD_REQUEST,
                "Expected Content-Type: application/x-tar".to_string(),
            ));
        }
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Expected Content-Type: application/x-tar".to_string(),
        ));
    }

    let mut archive = Archive::new(Cursor::new(body));

    let files = archive
        .entries()
        .map(|entries| entries.count())
        .unwrap_or(0);

    Ok((StatusCode::OK, files.to_string()))
}

async fn archive_files_size(
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Some(ct) = headers.get("Content-Type") {
        if ct != "application/x-tar" {
            return Err((
                StatusCode::BAD_REQUEST,
                "Expected Content-Type: application/x-tar".to_string(),
            ));
        }
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Expected Content-Type: application/x-tar".to_string(),
        ));
    }

    let mut archive = Archive::new(Cursor::new(body));

    let files_size = archive.entries().unwrap().fold(0u64, |acc, entry| {
        acc + entry.unwrap().header().size().unwrap()
    });

    Ok((StatusCode::OK, files_size.to_string()))
}
