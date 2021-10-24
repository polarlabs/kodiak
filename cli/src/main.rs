mod crud;

use std::str::FromStr;
use crate::crud::{create, read, update, delete, read_by_key};

use kodiak_core::unit::UnitType;

#[macro_use]
extern crate clap;
use clap::App;

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
            let unit_type = UnitType::from_str(sub_m.value_of("UNIT_TYPE").unwrap()).unwrap();
            let client = reqwest::Client::new();
            let u = create(client, unit_type, name.as_str()).await;

            if debug {
                println!("Debug: unit created {:?}", u);
            }
        }
        ("read", Some(sub_m)) => {
            let unit_type = UnitType::from_str(sub_m.value_of("UNIT_TYPE").unwrap()).unwrap();
            let client = reqwest::Client::new();

            if let Some(key) = sub_m.value_of("KEY") {
                let u = read_by_key(client, unit_type, key).await;

                if debug {
                    println!("Debug: unit read {:?}", u);
                }

            }
            else {
                let u = read(client, unit_type).await;

                if debug {
                    println!("Debug: units read {:?}", u);
                }
            }
        }
        ("update", Some(sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap().to_owned();
            let unit_type = UnitType::from_str(sub_m.value_of("UNIT_TYPE").unwrap()).unwrap();
            let payload = sub_m.value_of("PAYLOAD").unwrap();
            let client = reqwest::Client::new();
            let u = update(client, unit_type, key.as_str(), payload).await;

            if debug {
                println!("Debug: unit updated {:?}", u);
            }
        }
        ("delete", Some(sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap().to_owned();
            let unit_type = UnitType::from_str(sub_m.value_of("UNIT_TYPE").unwrap()).unwrap();
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
