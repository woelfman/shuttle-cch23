use axum::Json;
use serde::Serialize;

#[derive(Default, Serialize, Debug)]
pub struct Elf {
    pub elf: u32,
    #[serde(rename = "elf on a shelf")]
    pub elf_on_a_shelf: u32,
    #[serde(rename = "shelf with no elf on it")]
    pub shelf_with_no_elf: u32,
}

pub async fn elf(payload: String) -> Json<Elf> {
    const EOAS: &str = "elf on a shelf";
    let mut elves = Elf::default();
    let mut idx = 0;

    while let Some(i) = payload[idx..].find("elf") {
        elves.elf += 1;
        if payload[(idx + i)..].starts_with(EOAS) {
            elves.elf_on_a_shelf += 1;
        }
        idx += i + "elf".len();
    }

    idx = 0;
    while let Some(i) = payload[idx..].find("shelf") {
        idx += i + "shelf".len();
        if !payload[0..idx].ends_with(EOAS) {
            elves.shelf_with_no_elf += 1;
        }
    }

    Json(elves)
}
