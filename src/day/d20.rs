use std::io::Cursor;

use axum::{body::Bytes, http::StatusCode, response::IntoResponse, routing::post, Router};
use tar::Archive;
use tokio::process::Command;

pub fn get_routes() -> Router {
    Router::new()
        .route("/20/archive_files", post(archive_files))
        .route("/20/archive_files_size", post(archive_files_size))
        .route("/20/cookie", post(cookie))
}

async fn archive_files(body: Bytes) -> impl IntoResponse {
    let mut archive = Archive::new(Cursor::new(body));

    let files = archive
        .entries()
        .map(|entries| entries.count())
        .unwrap_or(0);

    files.to_string()
}

async fn archive_files_size(body: Bytes) -> impl IntoResponse {
    let mut archive = Archive::new(Cursor::new(body));

    let files_size = archive.entries().unwrap().fold(0u64, |acc, entry| {
        acc + entry.unwrap().header().size().unwrap()
    });

    files_size.to_string()
}

async fn cookie(body: Bytes) -> Result<impl IntoResponse, impl IntoResponse> {
    // Extract the archive to a temporary directory

    let dst =
        tempfile::tempdir().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut archive = Archive::new(Cursor::new(body));

    archive
        .unpack(&dst)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Use `git` to find commit

    let mut command = Command::new("git");
    let output = command
        .args(["log", "christmas", "-p", "--", "*santa.txt"])
        .current_dir(&dst)
        .output()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .stdout;
    let output = String::from_utf8(output)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for commit in output.split("commit ") {
        for line in commit.lines() {
            if line.starts_with('+') && line.contains("COOKIE") {
                for line in commit.lines() {
                    if line.starts_with("Author: ") {
                        let author = line
                            .strip_prefix("Author: ")
                            .unwrap()
                            .split(" <")
                            .next()
                            .unwrap();
                        let hash = commit.lines().next().unwrap();

                        return Ok(format!("{author} {hash}"));
                    }
                }
            }
        }
    }

    Err((
        StatusCode::UNPROCESSABLE_ENTITY,
        "commit not found".to_string(),
    ))
}
