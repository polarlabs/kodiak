pub mod io;
pub mod unit;

use unit::factory;
use unit::UnitType;
use unit::CRUD;

use std::collections::HashMap;

pub fn create<'a>(state: &'a mut HashMap<String, Box<dyn CRUD>>, unit_type: UnitType, name: &str) -> &'a Box<dyn CRUD> {
    let unit = factory(unit_type, name);
    let key = unit.key();
    state.insert(key.clone(), unit);
    state.get(key.as_str()).unwrap()
}

pub fn read<'a>(state: &'a HashMap<String, Box<dyn CRUD>>, key: &'a str) -> Option<&'a Box<dyn CRUD>> {
    if state.contains_key(key) {
        Some(state.get(key).unwrap().clone())
    } else {
        None
    }
}

pub fn update<'a>(state: &'a mut HashMap<String, Box<dyn CRUD>>, key: &'a str) -> Option<&'a Box<dyn CRUD>> {
    if state.contains_key(key) {
        let unit = state.get_mut(key).unwrap();
        unit.update();
        Some(unit)
    } else {
        None
    }
}

pub fn delete(state: &mut HashMap<String, Box<dyn CRUD>>, key: &str) -> Option<Box<dyn CRUD>> {
    state.remove(key)
}
