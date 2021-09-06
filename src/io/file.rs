use serde_json::{json, Map, Value};

use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};
use crate::unit::CRUD;
use std::collections::HashMap;
use std::ops::Deref;

// Creates a file
fn create(file_name: &str) -> File {
    // todo: implement error handling for file creation
    // todo: initialize file with {}
    File::create(file_name).unwrap()
}

// Reads a file into a Serde Map
pub fn read(file_name: &str) -> HashMap<String, Box<dyn CRUD>> {
    // Open file and handle involved errors
    let file = File::open(file_name.to_string());
    let mut file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => create(file_name),
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };

    // Read file into String
    let mut data = String::new();
    // todo: implement error handling for empty file
    file.read_to_string(&mut data);

    // Create Map from String
    //let event: Box<dyn WebEvent> = serde_json::from_str(json)?;
    let json: Result<Value, serde_json::Error> = serde_json::from_str(&data);
    let state = match json {
        Ok(json) => {
            let mut state2 = HashMap::new();
            let state = json.as_object().unwrap().clone();
            for object in state.iter() {
                state2.insert(object.0.clone(), serde_json::from_value(object.1.clone()).unwrap());
            }
            state2
        },
        Err(error) => {
            // todo: improve error handling, e.g. log parsing error, which might be a fatal error
            HashMap::new()
        },
    };

    state
}

pub fn write(file_name: &str, state: &HashMap<String, Box<dyn CRUD>>) {
    let mut state2 = Map::new();

    for object in state.iter() {
        state2.insert(object.0.clone(), json!(object.1));
    }

    let data = json!(state2).to_string();
    fs::write(file_name, data.as_bytes()).unwrap()

    // todo: beautify output to file
    // todo: add final line break to file

    // Open file and handle involved errors
    /*
    let file = File::open(file_name.to_string());
    let mut file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => create(file_name),
            other_error=> panic!("Error opening the file: {:?}", other_error),
        },
    };
    */
    /*
    let mut file = File::open(file_name.to_string()).unwrap();*/

    //file.write_all(data.as_bytes()).unwrap();
}
