use axum::{http::StatusCode, response::IntoResponse, Json};
use handlebars::{no_escape, Handlebars};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Payload {
    content: String,
}

pub async fn r#unsafe(
    Json(payload): Json<Payload>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(no_escape);
    let source = "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{content}}
  </body>
</html>";
    handlebars
        .register_template_string("t1", source)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok::<(StatusCode, String), (StatusCode, String)>((
        StatusCode::OK,
        handlebars.render("t1", &payload).unwrap(),
    ))
}

pub async fn safe(Json(payload): Json<Payload>) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut handlebars = Handlebars::new();
    let source = "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{content}}
  </body>
</html>";
    handlebars
        .register_template_string("t1", source)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok::<(StatusCode, String), (StatusCode, String)>((
        StatusCode::OK,
        handlebars.render("t1", &payload).unwrap(),
    ))
}
