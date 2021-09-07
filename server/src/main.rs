mod views;

use kodiak_core::io;
use kodiak_core::unit;
use kodiak_core::{create, read, update, delete};

use unit::factory;
use unit::UnitType;
use unit::CRUD;

use actix_web::{web, App as WebApp, HttpRequest, HttpServer, Responder};

use io::file::read as file_read;
use io::file::write as file_write;

#[macro_use]
extern crate clap;
use clap::App;

use std::collections::HashMap;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load clap YAML file
    let yaml = load_yaml!("cli.yml");
    let app_m = App::from(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();
    let debug = app_m.is_present("DEBUG");

    let mut state: HashMap<String, Box<dyn CRUD>> = file_read("./kodiak.file");

    let port = app_m.value_of("PORT").unwrap();

    HttpServer::new(|| {
        WebApp::new()
            .configure(views::factory)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await

    //file_write("./kodiak.file", &state);
}
