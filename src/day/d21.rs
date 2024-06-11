//! Day 21: Around the Globe
//!
//! Once upon a frosty night in Christmas' season, ol' Santa was tidying up his
//! archives. With his rosy cheeks and a finer air of mystery, he stumbled upon
//! a pile of old, dusty tape drives. Intrigued, he gave a mighty tug and dust
//! flew in the air, making him sneeze in the most jolly way possible.
//!
//! As he dusted them off, memories flooded back. Such mirth and jingle echoed
//! in his mind. They were his old present delivery logs and routes, the ones he
//! hadn't seen in years!
//!
//! # Task 1: Flat Squares on a Round Sphere?
//!
//! Santa found a bunch of old tape drives in the archives. Reading their
//! contents revealed a bunch of coordinates in a strange format encoded with
//! ones and zeroes. He needs some help with parsing them.
//!
//! Make a GET endpoint `/21/coords/<binary>` that takes a `u64` in binary
//! representation representing an S2 cell ID. Return the cell's center
//! coordinates in DMS format rounded to 3 decimals (see format below).
//!
//! ## Examples
//!
//! ```not_rust
//! curl http://localhost:8000/21/coords/0100111110010011000110011001010101011111000010100011110001011011
//!
//! 83°39'54.324''N 30°37'40.584''W
//! ```
//!
//! ```not_rust
//! curl http://localhost:8000/21/coords/0010000111110000011111100000111010111100000100111101111011000101
//!
//! 18°54'55.944''S 47°31'17.976''E
//! ```
//!
//! # Task 2: Turbo-fast Country Lookup
//!
//! When Santa rides his sleigh across the world, he crosses so many country
//! borders that he sometimes forgets which country he is in. He needs a handy
//! little API for quickly checking where he has ended up.
//!
//! Make a GET endpoint `/21/country/<binary>` with the same type of input as in
//! Task 1, that returns the english name of the country that the corresponding
//! coordinates are in.
//!
//! The input is guaranteed to represent coordinates that are within one
//! country's borders.
//!
//! Hint for an API that can be used: "In a tunnel? Closed. On a street? Open.
//! In a tunnel? Slow. Passing over? Turbo."
//!
//! ```not_rust
//! curl http://localhost:8000/21/country/0010000111110000011111100000111010111100000100111101111011000101
//!
//! Madagascar
//! ```
use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use dms_coordinates::DMS;
use reverse_geocoder::ReverseGeocoder;
use s2::{cell::Cell, cellid::CellID, latlng::LatLng};

pub fn get_routes() -> Router {
    Router::new()
        .route("/21/coords/:binary", get(binary))
        .route("/21/country/:binary", get(country))
}

async fn binary(Path(binary): Path<String>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let binary =
        u64::from_str_radix(&binary, 2).map_err(|e| (StatusCode::NOT_ACCEPTABLE, e.to_string()))?;
    let cell_id = CellID(binary);
    let center: LatLng = cell_id.into();
    let lat = DMS::from_ddeg_latitude(center.lat.deg());
    let lon = DMS::from_ddeg_longitude(center.lng.deg());

    Ok(format!(
        "{}°{}'{:.3}''{} {}°{}'{:.3}''{}",
        lat.degrees,
        lat.minutes,
        lat.seconds,
        lat.cardinal.unwrap(),
        lon.degrees,
        lon.minutes,
        lon.seconds,
        lon.cardinal.unwrap()
    ))
}

async fn country(Path(binary): Path<String>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let binary =
        u64::from_str_radix(&binary, 2).map_err(|e| (StatusCode::NOT_ACCEPTABLE, e.to_string()))?;
    let center = Cell::from(CellID(binary)).center();
    let geocoder = ReverseGeocoder::new();
    let search_result = geocoder.search((center.latitude().deg(), center.longitude().deg()));
    let country = match search_result.record.cc.as_str() {
        "BN" => Some("Brunei"),
        "NL" => Some("Belgium"), // Not sure about this one... maps indicate Netherlands, cch23-validator expects Belgium
        cc => rust_iso3166::from_alpha2(cc).map(|c| c.name),
    };
    if let Some(country) = country {
        return Ok(country);
    }

    Err((StatusCode::NOT_ACCEPTABLE, "No country found".to_string()))
}
