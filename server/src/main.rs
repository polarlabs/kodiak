use kodiak_core::io::file::{read as file_read, write as file_write};
use kodiak_core::unit::Unit;

#[macro_use]
extern crate clap;
use clap::App;

use std::collections::HashMap;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load clap YAML file
    let yaml = load_yaml!("cli.yml");
    let app_m = App::from(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();
    let _debug = app_m.is_present("DEBUG");

    let state: HashMap<String, Unit> = file_read("./kodiak.file");

    let port = app_m.value_of("PORT").unwrap();

    // Using a TcpListener allows to define the port to bind to, even to dynamic ports by using port 0
    // todo: error handling
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let _error = kodiak_interface::run(listener)?.await;

    file_write("./kodiak.file", &state);

    Ok(())
}
