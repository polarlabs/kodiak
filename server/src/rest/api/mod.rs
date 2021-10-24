use std::collections::HashMap;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetQ {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskQ {
    subject: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserQ {
    email: String,
}

pub fn factory(app: &mut web::ServiceConfig) {
    let base = Path::new("/rest/api/v1");

    app
        .route(&base.sub("/assets"), web::post().to(create_asset) )
        .route(&base.sub("/tasks"), web::post().to(create_task) )
        .route(&base.sub("/users"), web::post().to(create_user) )
        .route(&base.sub("/{unittype}"), web::get().to(read) )
        .route(&base.sub("/{unittype}/{key}"), web::get().to(read_by_key) )
        .route(&base.sub("/{unittype}"), web::put().to(update) )
        .route(&base.sub("/{unittype}/{key}"), web::delete().to(delete_by_key) );
}

fn create_asset(state: web::Data<AppState>, web::Query(unit): web::Query<AssetQ>, _req: HttpRequest) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    let state = data.deref_mut();

    println!("POST: asset {}", unit.name);

    let resp = create(state, UnitType::Asset, unit.name.as_str());
    file_write("./kodiak.file", &data);

    resp
}

fn create_task(state: web::Data<AppState>, web::Query(unit): web::Query<TaskQ>, _req: HttpRequest) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    let state = data.deref_mut();

    println!("POST: task {}", unit.subject);

    let resp = create(state, UnitType::Task, unit.subject.as_str());
    file_write("./kodiak.file", &data);

    resp
}

fn create_user(state: web::Data<AppState>, web::Query(unit): web::Query<UserQ>, _req: HttpRequest) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    let state = data.deref_mut();

    println!("POST: user {}", unit.email);

    let resp = create(state, UnitType::User, unit.email.as_str());
    file_write("./kodiak.file", &data);

    resp
}

fn create(state: &mut HashMap<String, Unit>, unittype: UnitType, name: &str) -> HttpResponse {
    let resp = match kodiak_core::create(state, unittype, name) {
        Some(unit) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json!(&unit))
        },
        None => HttpResponse::NotFound().finish(),
    };

    resp
}

fn read(state: web::Data<AppState>, web::Path(unittype): web::Path<UnitType>, _req: HttpRequest) -> HttpResponse {
    let data = state.data.lock().unwrap();
    let state = data.deref();

    println!("GET: unittype {:?}", unittype);

    let resp = HttpResponse::Ok()
        .content_type("application/json")
        .body(json!(kodiak_core::read(&state, unittype)));

    resp
}

fn read_by_key(state: web::Data<AppState>, web::Path((unittype, key)): web::Path<(UnitType, String)>, _req: HttpRequest) -> HttpResponse {
    let data = state.data.lock().unwrap();
    let state = data.deref();

    println!("GET: unittype {:?}, key {}", unittype, key);

    let resp = match kodiak_core::read_by_key(&state, unittype, key.as_str()) {
        Some(unit) => {
            HttpResponse::Ok()
                .content_type("application/json")
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

fn delete_by_key(state: web::Data<AppState>, web::Path((unittype, key)): web::Path<(UnitType, String)>, _req: HttpRequest) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    let state = data.deref();

    println!("DELETE: unittype {:?}, key {}", unittype, key);

    let resp = match kodiak_core::read_by_key(&state, unittype, key.as_str()) {
        Some(u) => {
            let body = json!(&u);

            data.remove(key.as_str());

            file_write("./kodiak.file", &data);

            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        }
        None => {
            HttpResponse::NotFound().finish()
        }
    };

    resp
}
