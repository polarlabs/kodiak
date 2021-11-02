pub mod views;
mod rest;

use kodiak_core::unit::Unit;
use kodiak_core::io::file::{read as file_read};

use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::dev::Server;

use std::net::TcpListener;
use std::sync::Mutex;
use std::collections::HashMap;

pub struct AppState {
    // Mutex required to mutate safely across threads
    data: Mutex<HashMap<String, Unit>>
}

pub async fn greet(req: HttpRequest) -> HttpResponse {
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok()
        .body(format!("Hello {}", name))
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Using a TcpListener allows to define the port to bind to, even to dynamic ports by using port 0
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let state = web::Data::new(AppState {
        data: Mutex::new(file_read("./kodiak.file")),
    });

    // todo: error handling
    // Moves state into the closure
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(views::factory)
            .configure(rest::api::factory)
            .route("/health", web::get().to(health))
            .service(fs::Files::new("/static", "./web").show_files_listing())
    })
        .listen(listener)?
        .run();

    Ok(server)
}
