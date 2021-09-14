use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use serde_json::json;

use kodiak_core::unit::{CRUD, Asset, Unit};

use kodiak_core::io::file::read as file_read;
use kodiak_core::io::file::write as file_write;
use super::Key;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetName {
    name: String,
}

pub async fn create(web::Query(asset): web::Query<AssetName>) -> HttpResponse {
    let asset = Asset::create(asset.name.as_str());

    let body = json!(&asset);

    let mut state: HashMap<String, Unit> = file_read("./kodiak.file");
    state.insert(asset.key(), Unit::Asset(asset));

    file_write("./kodiak.file", &state);

    HttpResponse::Ok()
        .body(body)
}

pub async fn read(web::Query(unit): web::Query<Key>) -> HttpResponse {

    let state: HashMap<String, Unit> = file_read("./kodiak.file");
    println!("Key: {:?}", state);

    if state.contains_key(unit.key.as_str()) {
        println!("Test");
        let unit = state.get(unit.key.as_str()).unwrap();
        HttpResponse::Ok()
            .body(json!(unit))
        /*
        match <dyn Any>::downcast_ref::<Asset>(unit) {
            Some(asset) => {
                HttpResponse::Ok()
                    .body(json!(asset))
            },
            None => HttpResponse::NotFound().finish(),
        }
        */
    }
    else {
        HttpResponse::NotFound().finish()
    }
}

// Do not forget to set Content-Type: application/json when requesting
pub async fn update(asset: web::Json<Asset>) -> HttpResponse {

    let mut state: HashMap<String, Unit> = file_read("./kodiak.file");

    println!("PUT");

    if state.contains_key(asset.key().as_str()) {
        let unit = state.get_mut(asset.key().as_str()).unwrap();
        match unit {
            Unit::Asset(asset) => { asset.update();}
            _ => {}
        }

        let body = json!(&unit);
        file_write("./kodiak.file", &state);

        HttpResponse::Ok()
            .body(body)
    }
    else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn delete(web::Query(unit): web::Query<Key>) -> HttpResponse {

    let mut state: HashMap<String, Unit> = file_read("./kodiak.file");

    println!("DELETE");
    if state.contains_key(unit.key.as_str()) {
        let asset = state.get(unit.key.as_str()).unwrap();
        let body = json!(&asset);

        state.remove(unit.key.as_str());
        file_write("./kodiak.file", &state);

        HttpResponse::Ok()
            .body(body)
    }
    else {
        HttpResponse::NotFound().finish()
    }
}
