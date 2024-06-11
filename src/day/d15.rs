//! Day 15: The Password Validator
//!
//! There had been a few naughty incidents where mischievous users had tinkered
//! with other children's wish lists, not respecting the goodwill and spirit of
//! the platform. It was clear: the website needed to add a layer of security to
//! protect the integrity of each child's wish list.
//!
//! The team behind the scenes, a dedicated crew of Santa's tech-savvy elves
//! (led by you), rolled up their sleeves. They decided to implement a homemade
//! password validation system that ensured that no Grinch could easily guess or
//! crack the password.
//!
//! # Task 1: Naughty or Nice Strings
//!
//! Now that children can sign up to the letter sending website, we need an
//! endpoint for validating their passwords.
//!
//! Create an endpoint at `/15/nice` that accepts POST requests with a JSON
//! payload containing a password string to validate.
//!
//! The rules at this endpoint are:
//!
//! * Nice Strings: Must contain at least three vowels (`aeiouy`), at least one
//! letter that appears twice in a row, and must not contain the substrings: `ab`,
//! `cd`, `pq`, or `xy`.
//! * Naughty Strings: Do not meet the criteria for nice strings.
//!
//! Return an appropriate HTTP status code and message (see below) indicating
//! whether the provided string is nice or naughty.
//!
//! ## Examples
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/15/nice \
//!   -H 'Content-Type: application/json' \
//!   -d '{"input": "hello there"}'
//!
//! # 200 OK
//! {"result":"nice"}
//! ```
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/15/nice \
//!   -H 'Content-Type: application/json' \
//!   -d '{"input": "abcd"}'
//!
//! # 400 Bad Request
//! {"result":"naughty"}
//! ```
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/15/nice \
//!   -H 'Content-Type: application/json' \
//!   -d '{Grinch? GRINCH!}'
//!
//! # 400 Bad Request
//! # response body does not matter
//! ```
//!
//! # Task 2: Game of the Year
//!
//! Santa thought this validation thing was so fun that it could be turned into
//! a game!
//!
//! Add a similar endpoint, POST `/15/game`, that has this set of rules:
//!
//! * **Nice Strings**: Must adhere to all the rules:
//!   * Rule 1: must be at least 8 characters long
//!   * Rule 2: must contain uppercase letters, lowercase letters, and digits
//!   * Rule 3: must contain at least 5 digits
//!   * Rule 4: all integers (sequences of consecutive digits) in the string must add up to 2023
//!   * Rule 5: must contain the letters `j`, `o`, and `y` in that order and in no other order
//!   * Rule 6: must contain a letter that repeats with exactly one other letter between them (like `xyx`)
//!   * Rule 7: must contain at least one unicode character in the range `[U+2980, U+2BFF]`
//!   * Rule 8: must contain at least one emoji
//!   * Rule 9: the hexadecimal representation of the sha256 hash of the string must end with an `a`
//! * **Naughty Strings**: Do not meet the criteria for nice strings.
//!
//! Check the string for the rules in the listed order. Return the corresponding
//! status code and reason (and naughty/nice result) based on which rule was
//! violated:
//!
//! | Rule broken | Status Code | Reason                 |
//! | ----------- | ----------- | ---------------------- |
//! | 1           | 400         | 8 chars                |
//! | 2           | 400         | more types of chars    |
//! | 3           | 400         | 55555                  |
//! | 4           | 400         | math is hard           |
//! | 5           | 406         | not joyful enough      |
//! | 6           | 451         | illegal: no sandwich   |
//! | 7           | 416         | outranged              |
//! | 8           | 426         | 😳                     |
//! | 9           | 418         | not a coffee brewer    |
//! | None        | 200         | that's a nice password |
//!
//! ## Examples
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/15/game \
//!   -H 'Content-Type: application/json' \
//!   -d '{"input": "password"}'
//!
//! # 400 Bad Request
//! {"result":"naughty","reason":"more types of chars"}
//! ```
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/15/game \
//!   -H 'Content-Type: application/json' \
//!   -d '{"input": "Password12345"}'
//!
//! # 400 Bad Request
//! {"result":"naughty","reason":"math is hard"}
//! ```
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/15/game \
//!   -H 'Content-Type: application/json' \
//!   -d '{"input": "23jPassword2000y"}'
//!
//! # 451 Unavailable For Legal Reasons
//! {"result":"naughty","reason":"illegal: no sandwich"}
//! ```
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use itertools::Itertools;
use serde::Deserialize;
use serde_json::json;
use unic::emoji::char::is_emoji;

/// Get Day 15 routes
///
/// * `/15/nice`
/// * `/15/game`
pub fn get_routes() -> Router {
    Router::new()
        .route("/15/nice", post(nice))
        .route("/15/game", post(game))
}

#[derive(Deserialize)]
struct Payload {
    input: String,
}

async fn nice(Json(payload): Json<Payload>) -> Result<impl IntoResponse, impl IntoResponse> {
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

async fn game(Json(payload): Json<Payload>) -> Result<impl IntoResponse, impl IntoResponse> {
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
