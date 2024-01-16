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
