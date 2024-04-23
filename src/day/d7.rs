use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use base64::engine::{general_purpose, Engine};
use serde_json::Value;

pub async fn decode(jar: CookieJar) -> Result<String, StatusCode> {
    if let Some(recipe) = jar.get("recipe") {
        general_purpose::URL_SAFE
            .decode(recipe.value())
            .ok()
            .and_then(|r| String::from_utf8(r).ok())
            .ok_or(StatusCode::NOT_ACCEPTABLE)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn bake(jar: CookieJar) -> Result<String, StatusCode> {
    if let Some(recipe) = jar.get("recipe") {
        let recipe_d = general_purpose::URL_SAFE
            .decode(recipe.value())
            .ok()
            .and_then(|r| String::from_utf8(r).ok())
            .ok_or(StatusCode::NOT_ACCEPTABLE)?;
        let json: Value =
            serde_json::from_str(&recipe_d).map_err(|_| StatusCode::NOT_ACCEPTABLE)?;
        let recipe = json
            .get("recipe")
            .ok_or(StatusCode::NOT_ACCEPTABLE)?
            .clone();
        let mut pantry = json
            .get("pantry")
            .ok_or(StatusCode::NOT_ACCEPTABLE)?
            .clone();

        let mut cookies = u64::MAX;

        // Get the number of cookies we can bake
        for (key, value) in recipe.as_object().unwrap() {
            if let Some(ingredient) = pantry.get(key) {
                if value.as_u64().unwrap_or_default() > 0 {
                    cookies = (ingredient.as_u64().unwrap_or_default()
                        / value.as_u64().unwrap_or(1))
                    .min(cookies);
                }
            }
        }

        // Subtract the ingredients used to make the cookies
        for (key, value) in recipe.as_object().unwrap() {
            if let Some(ingredient) = pantry.get_mut(key) {
                *ingredient = Value::Number(serde_json::Number::from(
                    ingredient.as_u64().unwrap() - cookies * value.as_u64().unwrap(),
                ));
            }
        }

        if cookies == u64::MAX {
            cookies = 0;
        }

        let result = serde_json::json!({
            "cookies": cookies,
            "pantry": pantry,
        })
        .to_string();

        Ok(result)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
