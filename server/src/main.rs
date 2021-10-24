#[macro_use]
extern crate clap;
use clap::App;

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

    let port = app_m.value_of("PORT").unwrap();

    // Using a TcpListener allows to define the port to bind to, even to dynamic ports by using port 0
    // todo: error handling
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let _error = kodiak_interface::run(listener)?.await;

    Ok(())
}
