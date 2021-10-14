use actix_web::{web, HttpResponse};
use serde_json::json;

use kodiak_core::unit::{CRUD, Task, Unit};
use kodiak_core::io::file::{write as file_write};

use crate::AppState;

// Do not forget to set Content-Type: application/json when requesting
pub async fn update(state: web::Data<AppState>, task: web::Json<Task>) -> HttpResponse {
    let mut data = state.data.lock().unwrap();

    println!("PUT Task");

    if data.contains_key(task.key().as_str()) {
        let unit = data.get_mut(task.key().as_str()).unwrap();
        match unit {
            Unit::Task(task) => { task.update();}
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
