use actix_web::{web, HttpResponse};
use serde_json::json;

use kodiak_core::unit::{CRUD, User, Unit};
use kodiak_core::io::file::{write as file_write};

use crate::AppState;

// Do not forget to set Content-Type: application/json when requesting
pub async fn update(state: web::Data<AppState>, user: web::Json<User>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();

    println!("PUT User");

    if data.contains_key(user.key().as_str()) {
        let unit = data.get_mut(user.key().as_str()).unwrap();
        match unit {
            Unit::User(user) => { user.update();}
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
