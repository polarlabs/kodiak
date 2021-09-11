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

    // Read file into String.
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => { println!("Reading data from file {} succeeded.", file_name)}
        Err(e) => { panic!("Reading data from file {} failed: {:?}", file_name, e) }
    }

    // Deserialize into Serde Map from String and create state by iterating over it.
    let json: Result<Value, serde_json::Error> = serde_json::from_str(&data);
    let state = match json {
        Ok(json) => {
            println!("Parsing state from {} succeeded.", file_name);
            let mut state = HashMap::new();
            let map = json.as_object().unwrap().clone();
            for record in map.iter() {
                state.insert(record.0.clone(), serde_json::from_value(record.1.clone()).unwrap());
            }
            state
        },
        Err(e) => {
            // todo: Error occurs also when file is empty / newly created.
            // Log error and return empty state.
            println!("Parsing state from {} failed: {:?}.", file_name, e);
            HashMap::new()
        },
    };

    state
}

///
/// Securely persisting state to a file is not easy. To keep data consistent ``write`` implements the following measures:
/// 1. Lock to prevent simultaneous writes.
/// 2. Write state to a temporary file first, then rename it overwriting existing state.
///
/// Readability for humans is ensured by the following features:
/// 1. Human readable JSON format with indentation and newlines.
/// 2. Trailing newline to improve readability in terminal.
///
pub fn write(file_name: &str, state: &HashMap<String, Box<dyn CRUD>>) {
    let lock = "kodiak.lock";

    // Create lock, fail if lock already exists.
    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(lock) {
        Ok(_) => {
            // Write to temporary file
            let temp = file_name.to_owned() + "-temp";

            // Create new or truncate existing file.
            let mut file = match fs::File::create(temp.as_str()) {
                Ok(file) => file,
                Err(e) => panic!("Error creating temporary file {}: {:?}", temp.as_str(), e),
            };

            // Create a Serde Map and fill it by iterating over state.
            let mut map = Map::new();
            for record in state.iter() {
                map.insert(record.0.clone(), json!(record.1));
            }

            // Serialize Serde Map and write it to file.
            let data = json!(map);
            let data = serde_json::to_string_pretty(&data).unwrap() + "\n";
            match file.write_all(data.as_bytes()) {
                Ok(_) => {
                    match fs::rename(temp.as_str(), file_name) {
                        Ok(_) => println!("State persisted to disk in file {}", file_name),
                        Err(e) => panic!("Error renaming temporary {} to {}: {:?}", temp.as_str(), file_name, e),
                    };
                },
                Err(e) => panic!("Error writing to file {}: {:?}", temp.as_str(), e),
            }

            // Remove lock
            // todo: error handling
            let _ = fs::remove_file(lock);
        }
        Err(e) => {
            println!("State persisting failed: {}. Locked.", e);
        }
    }
}
