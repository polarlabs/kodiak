use actix_web::{HttpResponse};
use serde_json::json;

use kodiak_core::unit::{User, Unit, CRUD};
use kodiak_core::io::file::{write as file_write};

use std::collections::HashMap;

pub fn update(state: &mut HashMap<String, Unit>, user: User) -> HttpResponse {
    println!("PUT User");

    let unit = state.get_mut(user.key().as_str());
    match unit {
        Some(Unit::User(user)) => {
            user.update();
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
