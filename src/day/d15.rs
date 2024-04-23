use axum::{http::StatusCode, response::IntoResponse, Json};
use itertools::Itertools;
use serde::Deserialize;
use serde_json::json;
use unic::emoji::char::is_emoji;

#[derive(Deserialize)]
pub struct Payload {
    input: String,
}

pub async fn nice(Json(payload): Json<Payload>) -> Result<impl IntoResponse, impl IntoResponse> {
    let naughty = ["ab", "cd", "pq", "xy"];
    let nice = ['a', 'e', 'i', 'o', 'u', 'y'];

    // Must not contain `naughty`
    if naughty.iter().any(|s| payload.input.contains(s)) {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"result": "naughty"}))));
    }

    // Must contain at least 3 `nice` vowels
    if payload
        .input
        .chars()
        .fold(0, |acc, c| acc + i32::from(nice.contains(&c)))
        < 3
    {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"result": "naughty"}))));
    }

    // Must contain two consecutive `nice` vowels
    if !payload
        .input
        .chars()
        .tuple_windows::<(_, _)>()
        .any(|(a, b)| a.is_alphabetic() && a == b)
    {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"result": "naughty"}))));
    }

    Ok((StatusCode::OK, Json(json!({"result": "nice"}))))
}

pub async fn game(Json(payload): Json<Payload>) -> Result<impl IntoResponse, impl IntoResponse> {
    // Rule 1: must be at least 8 characters long
    if payload.input.chars().count() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"result": "naughty", "reason": "8 chars"})),
        ));
    }

    // Rule 2: must contain uppercase letters, lowercase letters, and digits
    if !payload.input.chars().any(char::is_uppercase)
        || !payload.input.chars().any(char::is_lowercase)
        || !payload.input.chars().any(|c| c.is_ascii_digit())
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"result": "naughty", "reason": "more types of chars"})),
        ));
    }

    // Rule 3: must contain at least 5 digits
    if payload
        .input
        .chars()
        .fold(0, |acc, c| acc + i32::from(c.is_ascii_digit()))
        < 5
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"result": "naughty", "reason": "55555"})),
        ));
    }

    // Rule 4: all integers (sequences of consecutive digits) in the string must add up to 2023
    let re = regex::Regex::new(r"\d+").unwrap();
    if re
        .find_iter(&payload.input)
        .filter_map(|s| s.as_str().parse::<u32>().ok())
        .sum::<u32>()
        != 2023
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"result": "naughty", "reason": "math is hard"})),
        ));
    }

    // Rule 5: must contain the letters j, o, and y in that order and in no other order
    if payload
        .input
        .chars()
        .filter(|c| "joy".contains(*c))
        .collect::<String>()
        != "joy"
    {
        return Err((
            StatusCode::NOT_ACCEPTABLE,
            Json(json!({"result": "naughty", "reason": "not joyful enough"})),
        ));
    }

    // Rule 6: must contain a letter that repeats with exactly one other letter between them (like xyx)
    if !payload
        .input
        .chars()
        .tuple_windows::<(_, _, _)>()
        .any(|(a, b, c)| a.is_alphabetic() && b.is_alphabetic() && a == c && a != b)
    {
        return Err((
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            Json(json!({"result": "naughty", "reason": "illegal: no sandwich"})),
        ));
    }

    // Rule 7: must contain at least one unicode character in the range [U+2980, U+2BFF]
    if !payload
        .input
        .chars()
        .any(|c| ('\u{2980}'..='\u{2bff}').contains(&c))
    {
        return Err((
            StatusCode::RANGE_NOT_SATISFIABLE,
            Json(json!({"result": "naughty", "reason": "outranged"})),
        ));
    }

    // Rule 8: must contain at least one emoji
    if !payload
        .input
        .chars()
        .any(|c| is_emoji(c) && !c.is_numeric())
    {
        return Err((
            StatusCode::UPGRADE_REQUIRED,
            Json(json!({"result": "naughty", "reason": "😳"})),
        ));
    }

    // Rule 9: the hexadecimal representation of the sha256 hash of the string must end with an 'a'
    if !sha256::digest(&payload.input).ends_with('a') {
        return Err((
            StatusCode::IM_A_TEAPOT,
            Json(json!({"result": "naughty", "reason": "not a coffee brewer"})),
        ));
    }

    Ok((
        StatusCode::OK,
        Json(json!({"result": "nice", "reason": "that's a nice password"})),
    ))
}
