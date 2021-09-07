use crate::unit::CRUD;

use serde_json::{json, Map, Value};

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};

pub fn read(file_name: &str) -> HashMap<String, Box<dyn CRUD>> {
    // Open existing or create new file and handle involved errors
    let mut file = match fs::OpenOptions::new().create(true).read(true).write(true).open(file_name) {
        Ok(file) => file,
        Err(e) => panic!("Error opening / creating file {}: {:?}", file_name, e),
    };

    // Read file into String
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => { println!("Reading state from {} succeeded.", file_name)}
        Err(e) => { panic!("Fatal {}", e) }
    }

    // Create Map from String
    //let event: Box<dyn WebEvent> = serde_json::from_str(json)?;
    let json: Result<Value, serde_json::Error> = serde_json::from_str(&data);
    let state = match json {
        Ok(json) => {
            let mut state = HashMap::new();
            let map = json.as_object().unwrap().clone();
            for record in map.iter() {
                state.insert(record.0.clone(), serde_json::from_value(record.1.clone()).unwrap());
            }
            state
        },
        Err(_error) => {
            // todo: improve error handling, e.g. log parsing error, which might be a fatal error
            HashMap::new()
        },
    };

    state
}

pub fn write(file_name: &str, state: &HashMap<String, Box<dyn CRUD>>) {
    // Open existing or create new file and handle involved errors
    let mut file = match fs::OpenOptions::new().create(true).read(true).write(true).open(file_name) {
        Ok(file) => file,
        Err(e) => panic!("Error opening / creating file {}: {:?}", file_name, e),
    };

    let mut map = Map::new();
    for record in state.iter() {
        map.insert(record.0.clone(), json!(record.1));
    }

    let data = json!(map).to_string();
    match file.write_all(data.as_bytes()) {
        Ok(()) => println!("State persisted to disk in file {}", file_name),
        Err(e) => panic!("Error writing to file {}: {:?}", file_name, e),
    }

    // todo: beautify output to file
    // todo: add final line break to file
}
