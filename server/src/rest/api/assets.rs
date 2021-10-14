use actix_web::{web, HttpResponse};
use serde_json::json;

use kodiak_core::unit::{CRUD, Asset, Unit};
use kodiak_core::io::file::{write as file_write};

use crate::AppState;

// Do not forget to set Content-Type: application/json when requesting
pub fn update(state: web::Data<AppState>, asset: web::Json<Asset>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();

    println!("PUT Asset");

    if data.contains_key(asset.key().as_str()) {
        let unit = data.get_mut(asset.key().as_str()).unwrap();
        match unit {
            Unit::Asset(asset) => { asset.update();}
            _ => {}
        }
        let body = json!(&unit);

        file_write("./kodiak.file", &data);

        HttpResponse::Ok()
            .body(body)
    }
    else {
        HttpResponse::NotFound().finish()
    }
}
