use axum::{extract, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Deer {
    pub name: String,
    pub strength: i32,
    pub speed: Option<f32>,
    pub height: Option<u16>,
    pub antler_width: Option<u16>,
    pub snow_magic_power: Option<u16>,
    pub favorite_food: Option<String>,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    pub candies_eaten_yesterday: Option<u8>,
}

#[derive(Serialize, Default, Debug)]
pub struct Contest {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn strength(extract::Json(payload): extract::Json<Vec<Deer>>) -> String {
    payload
        .iter()
        .fold(0i32, |acc, deer| acc + deer.strength)
        .to_string()
}

pub async fn contest(extract::Json(payload): extract::Json<Vec<Deer>>) -> Json<Contest> {
    let mut fastest = 0;
    let mut tallest = 0;
    let mut magician = 0;
    let mut consumer = 0;

    for (idx, deer) in payload.iter().enumerate() {
        if deer.speed.unwrap_or_default() > payload[fastest].speed.unwrap_or_default() {
            fastest = idx;
        }
        if deer.height.unwrap_or_default() > payload[tallest].height.unwrap_or_default() {
            tallest = idx;
        }
        if deer.snow_magic_power.unwrap_or_default()
            > payload[magician].snow_magic_power.unwrap_or_default()
        {
            magician = idx;
        }
        if deer.candies_eaten_yesterday.unwrap_or_default()
            > payload[consumer]
                .candies_eaten_yesterday
                .unwrap_or_default()
        {
            consumer = idx;
        }
    }

    let response = Contest {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            payload[fastest].strength, payload[fastest].name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            payload[tallest].name,
            payload[tallest].antler_width.unwrap_or_default()
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            payload[magician].name,
            payload[magician].snow_magic_power.unwrap_or_default()
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            payload[consumer].name,
            payload[consumer].favorite_food.as_ref().unwrap()
        ),
    };
    Json(response)
}
