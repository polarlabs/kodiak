use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use serde_json::json;

use kodiak_core::unit::{CRUD, Asset, Unit};
use kodiak_core::io::file::{write as file_write};
use super::Key;

use crate::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetName {
    name: String,
}

pub async fn create(state: web::Data<AppState>, web::Query(asset): web::Query<AssetName>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    let asset = Asset::create(asset.name.as_str());

    println!("POST: key {}", asset.key());

    let body = json!(&asset);

    data.insert(asset.key(), Unit::Asset(asset));

    file_write("./kodiak.file", &data);

    HttpResponse::Ok()
        .body(body)
}

pub async fn read(state: web::Data<AppState>, web::Query(unit): web::Query<Key>) -> HttpResponse {
    let data = state.data.lock().unwrap();

    println!("GET: key {}", unit.key.as_str());

    if data.contains_key(unit.key.as_str()) {
        let unit = data.get(unit.key.as_str()).unwrap();

        HttpResponse::Ok()
            .body(json!(unit))
    }
    else {
        HttpResponse::NotFound().finish()
    }
}

// Do not forget to set Content-Type: application/json when requesting
pub async fn update(state: web::Data<AppState>, asset: web::Json<Asset>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();

    println!("PUT");

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

pub async fn delete(state: web::Data<AppState>, web::Query(unit): web::Query<Key>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();

    println!("DELETE");

    if data.contains_key(unit.key.as_str()) {
        let asset = data.get(unit.key.as_str()).unwrap();
        let body = json!(&asset);

        data.remove(unit.key.as_str());

        file_write("./kodiak.file", &data);

        HttpResponse::Ok()
            .body(body)
    }
    else {
        HttpResponse::NotFound().finish()
    }
}
