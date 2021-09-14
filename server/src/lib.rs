pub mod views;
mod rest;

use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

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

    // todo: error handling
    let server = HttpServer::new(|| {
        App::new()
            .configure(views::factory)
            .configure(rest::api::factory)
            .route("/health", web::get().to(health))
            .route("/{name}", web::get().to(greet))
            .route("/", web::get().to(greet))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
