use actix_web::{web, HttpRequest, HttpResponse};
use crate::views::Path;

use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::AppState;
use kodiak_core::unit::UnitType;
use kodiak_core::unit::{CRUD, Asset, Task, User, Unit};
use kodiak_core::io::file::{write as file_write};
use std::ops::DerefMut;

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

    let unittype = match req.match_info().get("unittype").unwrap() {
        "assets" => Some(UnitType::Asset),
        "tasks" => Some(UnitType::Task),
        "users" => Some(UnitType::User),
        // todo: error handling
        &_ => { None }
    };

    let resp = match unittype {
        Some(UnitType::Asset) => {
            let unit = Asset::create(unit.name.as_str());
            let body = json!(Unit::Asset(unit.clone()));
            data.insert(unit.key(), Unit::Asset(unit));
            HttpResponse::Ok()
                .body(body)
        }
        Some(UnitType::Task) => {
            let unit = Task::create(unit.name.as_str());
            let body = json!(Unit::Task(unit.clone()));
            data.insert(unit.key(), Unit::Task(unit));
            HttpResponse::Ok()
                .body(body)
        },
        Some(UnitType::User) => {
            let unit = User::create(unit.name.as_str());
            let body = json!(Unit::User(unit.clone()));
            data.insert(unit.key(), Unit::User(unit));
            HttpResponse::Ok()
                .body(body)
        }
        // todo: error handling
        None => HttpResponse::NotFound().finish()
    };

    file_write("./kodiak.file", &data);

    resp
}

fn read(state: web::Data<AppState>, req: HttpRequest, web::Query(unit): web::Query<Key>) -> HttpResponse {

    let unittype = match req.match_info().get("unittype").unwrap() {
        "assets" => Some(UnitType::Asset),
        "tasks" => Some(UnitType::Task),
        "users" => Some(UnitType::User),
        // todo: error handling
        &_ => { None }
    };

    println!("GET: key {}", unit.key.as_str());

    let unit = get_unit_by_unittype(&state, unittype.unwrap(), unit.key.as_str());
    match unit {
        Some(Unit::Asset(_)) => {
            HttpResponse::Ok()
                .body(json!(unit))
        }
        Some(Unit::Task(_)) => {
            HttpResponse::Ok()
                .body(json!(unit))
        }
        Some(Unit::User(_)) => {
            HttpResponse::Ok()
                .body(json!(unit))
        }
        None => {
            HttpResponse::NotFound().finish()
        }
    }
}

fn get_unit_by_unittype(state: &web::Data<AppState>, unittype: UnitType, key: &str) -> Option<Unit> {
    let data = state.data.lock().unwrap();

    match data.get(key) {
        Some(u) => {
            let unit = u.clone();

            match unit {
                Unit::Asset(_) => {
                    if unittype == UnitType::Asset { Some(unit) } else { None }
                }
                Unit::Task(_) => {
                    if unittype == UnitType::Task { Some(unit) } else { None }
                }
                Unit::User(_) => {
                    if unittype == UnitType::User { Some(unit) } else { None }
                }
            }
        }
        None => { None }
    }
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

    let unittype = match req.match_info().get("unittype").unwrap() {
        "assets" => Some(UnitType::Asset),
        "tasks" => Some(UnitType::Task),
        "users" => Some(UnitType::User),
        // todo: error handling
        &_ => { None }
    };

    println!("DELETE: key {}", unit.key.as_str());

    let key = unit.key.as_str();
    let unit = get_unit_by_unittype(&state, unittype.unwrap(), key);
    match unit {
        Some(u) => {
            let mut data = state.data.lock().unwrap();
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
