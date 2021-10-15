use actix_web::{HttpResponse};
use serde_json::json;

use kodiak_core::unit::{Asset, Unit, CRUD};
use kodiak_core::io::file::{write as file_write};

use std::collections::HashMap;

pub fn update(state: &mut HashMap<String, Unit>, asset: Asset) -> HttpResponse {
    println!("PUT Asset");

    let unit = state.get_mut(asset.key().as_str());
    match unit {
        Some(Unit::Asset(asset)) => {
            asset.update();
            let body = json!(&unit);
            file_write("./kodiak.file", &state);

            HttpResponse::Ok()
                .body(body)

        }
        None => {
            HttpResponse::NotFound().finish()
        }
        _ => {
            HttpResponse::NotFound().finish()
        }
    }
}
