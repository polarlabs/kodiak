mod create;
use crate::create::{create, read, update, delete};

use kodiak_core::io::file::{read as file_read, write as file_write};

use kodiak_core::unit::{Unit, UnitType};

#[macro_use]
extern crate clap;
use clap::App;

use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Load clap YAML file
    let yaml = load_yaml!("cli.yml");
    let app_m = App::from(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();
    let debug = app_m.is_present("DEBUG");

    match app_m.subcommand() {
        ("create", Some(sub_m)) => {
            let name = sub_m.value_of("NAME").unwrap().to_owned();
            // todo: deserialization of UnitType
            let unit_type = match sub_m.value_of("UNIT_TYPE").unwrap() {
                "asset" => UnitType::Asset,
                "task" => UnitType::Task,
                "user" => UnitType::User,
                _ => { unreachable!() }
            };
            let client = reqwest::Client::new();
            let u = create(client, unit_type, name.as_str()).await;

            if debug {
                println!("Debug: unit created {:?}", u);
            }
        }
        ("read", Some(sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap().to_owned();
            // todo: deserialization of UnitType
            let unit_type = match sub_m.value_of("UNIT_TYPE").unwrap() {
                "asset" => UnitType::Asset,
                "task" => UnitType::Task,
                "user" => UnitType::User,
                _ => { unreachable!() }
            };
            let client = reqwest::Client::new();
            let u = read(client, unit_type, key.as_str()).await;

            if debug {
                println!("Debug: unit read {:?}", u);
            }
        }
        ("update", Some(sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap().to_owned();
            // todo: deserialization of UnitType
            let unit_type = match sub_m.value_of("UNIT_TYPE").unwrap() {
                "asset" => UnitType::Asset,
                "task" => UnitType::Task,
                "user" => UnitType::User,
                _ => { unreachable!() }
            };
            let client = reqwest::Client::new();
            let u = update(client, unit_type, key.as_str()).await;

            if debug {
                println!("Debug: unit updated {:?}", u);
            }
        }
        ("delete", Some(sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap().to_owned();
            // todo: deserialization of UnitType
            let unit_type = match sub_m.value_of("UNIT_TYPE").unwrap() {
                "asset" => UnitType::Asset,
                "task" => UnitType::Task,
                "user" => UnitType::User,
                _ => { unreachable!() }
            };
            let client = reqwest::Client::new();
            let u = delete(client, unit_type, key.as_str()).await;

            if debug {
                println!("Debug: unit deleted {:?}", u);
            }
        }
        _ => {
            unreachable!()
        }
    }
}
