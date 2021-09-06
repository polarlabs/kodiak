mod io;
mod unit;

use unit::factory;
use unit::UnitType;
use unit::CRUD;
use unit::{Asset, Task, User};

use io::file::read as file_read;
use io::file::write as file_write;

#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches};

#[macro_use]
extern crate serde_json;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::ops::Deref;

fn create<'a>(state: &'a mut HashMap<String, Box<dyn CRUD>>, unit_type: UnitType, name: &str) -> &'a Box<dyn CRUD> {
    let mut t1 = factory(unit_type);
    t1.create(name);
    let key = t1.key();
    state.insert(t1.key().clone(), t1);
    state.get(key.as_str()).unwrap()
}

fn read<'a>(state: &'a HashMap<String, Box<dyn CRUD>>, key: &'a str) -> Option<&'a Box<dyn CRUD>> {
    if state.contains_key(key) {
        Some(state.get(key).unwrap().clone())
    } else {
        None
    }
}

fn update<'a>(state: &'a mut HashMap<String, Box<dyn CRUD>>, key: &'a str) -> Option<&'a Box<dyn CRUD>> {
    if state.contains_key(key) {
        Some(state.get_mut(key).unwrap())
    } else {
        None
    }
}

fn delete(state: &mut HashMap<String, Box<dyn CRUD>>, key: &str) -> Option<Box<dyn CRUD>> {
    state.remove(key)
}

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
                Some(v) => println!("Unit {} deleted.", key),
                None => println!("Unit {} not found.", key),
            }
        }
        _ => {
            unreachable!()
        }
    }

    /*
    let a = Asset::new(Some("HOST001"));
    println!("{:?}", a);

    let t = Task::new(Some("Install Server HOST002"), Some("Open"));
    println!("{:?}", t);

    let u = User::new(Some("tobias.mucke@gmail.com"));
    println!("{:?}", u);

    state.insert(a.key(), json!(a));
    state.insert(t.key(), json!(t));
    state.insert(u.key(), json!(u));
    */
    /*
    let mut state: HashMap<&str, Box<dyn CRUD>> = HashMap::new();
    state.insert(t1.key, t1);
     */

    file_write("./kodiak.file", &state);
}
