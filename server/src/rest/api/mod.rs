use actix_web::{web, HttpRequest, HttpResponse};
use crate::views::Path;

use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::AppState;
use kodiak_core::unit::{Unit, UnitType};
use kodiak_core::io::file::{write as file_write};
use std::ops::{Deref, DerefMut};

mod assets;
mod tasks;
mod users;

#[derive(Deserialize)]
pub struct Key {
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    name: String,
}

pub fn factory(app: &mut web::ServiceConfig) {
    let base = Path::new("/rest/api/v1");

    app
        .route(&base.sub("/{unittype}"), web::post().to(create) )
        .route(&base.sub("/{unittype}"), web::get().to(read) )
        .route(&base.sub("/{unittype}"), web::delete().to(delete) )
        .route(&base.sub("/{unittype}"), web::put().to(update) );
}

fn create(state: web::Data<AppState>, req: HttpRequest, web::Query(unit): web::Query<Name>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    let state = data.deref_mut();

    let unittype = match req.match_info().get("unittype").unwrap() {
        "assets" => Some(UnitType::Asset),
        "tasks" => Some(UnitType::Task),
        "users" => Some(UnitType::User),
        // todo: error handling
        &_ => { None }
    };

    let resp = match kodiak_core::create(state, unittype.unwrap(), unit.name.as_str()) {
        Some(unit) => {
            HttpResponse::Ok().body(json!(&unit))
        },
        None => HttpResponse::NotFound().finish(),
    };

    file_write("./kodiak.file", &data);

    resp
}

fn read(state: web::Data<AppState>, req: HttpRequest, web::Query(unit): web::Query<Key>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    let state = data.deref_mut();

    let unittype = match req.match_info().get("unittype").unwrap() {
        "assets" => Some(UnitType::Asset),
        "tasks" => Some(UnitType::Task),
        "users" => Some(UnitType::User),
        // todo: error handling
        &_ => { None }
    };

    println!("GET: key {}", unit.key.as_str());

    let resp = match kodiak_core::read_by_unittype(&state, unittype.unwrap(), unit.key.as_str()) {
        Some(unit) => {
            HttpResponse::Ok()
                .body(json!(unit))
        }
        // todo: error handling
        None => HttpResponse::NotFound().finish()
    };

    resp
}

// Do not forget to set Content-Type: application/json when requesting
pub fn update(state: web::Data<AppState>, unit: web::Json<Unit>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    let state = data.deref_mut();

    println!("PUT");

    let resp = match unit.into_inner() {
        Unit::Asset(asset) => {
            assets::update(state, asset)
        }
        Unit::Task(task) => {
            tasks::update(state, task)
        }
        Unit::User(user) => {
            users::update(state, user)
        }
    };

    resp
}

fn delete(state: web::Data<AppState>, req: HttpRequest, web::Query(unit): web::Query<Key>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    let state = data.deref();

    let unittype = match req.match_info().get("unittype").unwrap() {
        "assets" => Some(UnitType::Asset),
        "tasks" => Some(UnitType::Task),
        "users" => Some(UnitType::User),
        // todo: error handling
        &_ => { None }
    };

    println!("DELETE: key {}", unit.key.as_str());

    let key = unit.key.as_str();
    let unit = kodiak_core::read_by_unittype(&state, unittype.unwrap(), key);
    match unit {
        Some(u) => {
            let body = json!(&u);

            data.remove(key);

            file_write("./kodiak.file", &data);

            HttpResponse::Ok()
                .body(body)
        }
        None => {
            HttpResponse::NotFound().finish()
        }
    }
}
