use actix_web::{HttpResponse};
use serde_json::json;

use kodiak_core::unit::{Task, Unit, CRUD};
use kodiak_core::io::file::{write as file_write};

use std::collections::HashMap;

pub fn update(state: &mut HashMap<String, Unit>, task: Task) -> HttpResponse {
    println!("PUT Task");

    let unit = state.get_mut(task.key().as_str());
    match unit {
        Some(Unit::Task(task)) => {
            task.update();
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
