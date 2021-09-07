use kodiak_core::io;
use kodiak_core::unit;
use kodiak_core::{create, read, update, delete};

use unit::UnitType;
use unit::CRUD;

use io::file::read as file_read;
use io::file::write as file_write;

#[macro_use]
extern crate clap;
use clap::App;

use std::collections::HashMap;

fn main() {
    // Load clap YAML file
    let yaml = load_yaml!("cli.yml");
    let app_m = App::from(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();
    let debug = app_m.is_present("DEBUG");

    let mut state: HashMap<String, Box<dyn CRUD>> = file_read("./kodiak.file");

    match app_m.subcommand() {
        ("create", Some(sub_m)) => {
            let name = sub_m.value_of("NAME").unwrap().to_owned();
            let unit_type = match sub_m.value_of("UNIT_TYPE").unwrap() {
                "asset" => UnitType::Asset,
                "task" => UnitType::Task,
                "user" => UnitType::User,
                _ => { unreachable!() }
            };
            let u = create(&mut state, unit_type, name.as_str());

            if debug {
                println!("Debug: unit created {:?}", u);
            }
        }
        ("read", Some(sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap().to_owned();
            let u = read(&mut state, key.as_str());

            println!("Unit key: {:?}", u);

            if debug {
                println!("Debug: units in database {:?}", state);
            }
        }
        ("update", Some(sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap().to_owned();
            let u = read(&mut state, key.as_str()).unwrap();

            println!("Unit before update: {:?}", u);
            let u = update(&mut state, key.as_str());
            println!("Unit after update: {:?}", u);
        }
        ("delete", Some(sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap().to_owned();
            match delete(&mut state, key.as_str()) {
                Some(_unit) => println!("Unit {} deleted.", key),
                None => println!("Unit {} not found.", key),
            }
        }
        _ => {
            unreachable!()
        }
    }

    file_write("./kodiak.file", &state);
}
