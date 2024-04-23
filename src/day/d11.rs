use std::io::Cursor;

use axum::{extract::Multipart, http::StatusCode};
use image::GenericImageView;

pub async fn red_pixels(mut multipart: Multipart) -> Result<String, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "image" {
            let data = field.bytes().await.unwrap();
            let decoder = image::io::Reader::new(Cursor::new(data))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            let magical_red = decoder.pixels().fold(0u64, |acc, (_x, _y, p)| {
                if u16::from(p[0]) > u16::from(p[1]) + u16::from(p[2]) {
                    acc + 1
                } else {
                    acc
                }
            });

            return Ok(magical_red.to_string());
        }
    }

    Err(StatusCode::BAD_REQUEST)
}
