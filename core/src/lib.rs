pub mod io;
pub mod unit;

use unit::{Unit, UnitType, Asset, Task, User, CRUD};

use std::collections::HashMap;

pub fn create(state: &mut HashMap<String, Unit>, unit_type: UnitType, name: &str) -> Option<Unit> {

    match unit_type {
        UnitType::Unit => {
            None
        }
        UnitType::Asset => {
            let unit = Asset::create(name);
            let key = unit.key();
            state.insert(key.clone(), Unit::Asset(unit.clone()));
            Some(Unit::Asset(unit))
        }
        UnitType::Task => {
            let unit = Task::create(name);
            let key = unit.key();
            state.insert(key.clone(), Unit::Task(unit.clone()));
            Some(Unit::Task(unit))
        }
        UnitType::User => {
            let unit = User::create(name);
            let key = unit.key();
            state.insert(key.clone(), Unit::User(unit.clone()));
            Some(Unit::User(unit))
        }
    }
}

//
// Returns a unit of defined type
// If unittype is Unit, a Unit of any type might be returned
//
pub fn read<'a>(state: &'a HashMap<String, Unit>, unittype: UnitType) -> Vec<&'a Unit> {

    match unittype {
        UnitType::Unit => {
            state.values().collect::<Vec<&Unit>>()
        }
        _ => {
            state.values().filter(|unit| unit.is_a(&unittype)).collect::<Vec<&Unit>>()
        }
    }
}

//
// Returns a unit of defined type
// If unittype is Unit, a Unit of any type might be returned
//
pub fn read_by_key<'a>(state: &'a HashMap<String, Unit>, unittype: UnitType, key: &str) -> Option<&'a Unit> {

    match unittype {
        UnitType::Unit => {
            state.get(key)
        }
        _ => {
            match state.get(key) {
                Some(unit) => {
                    match unit {
                        Unit::Asset(_) => {
                            if unittype == UnitType::Asset { Some(unit) } else { None }
                        }
                        Unit::Task(_) => {
                            if unittype == UnitType::Task { Some(unit) } else { None }
                        }
                        Unit::User(_) => {
                            if unittype == UnitType::User { Some(unit) } else { None }
                        }
                    }
                }
                None => { None }
            }
        }
    }
}

pub fn update<'a>(state: &'a mut HashMap<String, Unit>, key: &str) -> Option<&'a Unit> {

    match state.get_mut(key) {
        Some(unit) => {
            match unit {
                Unit::Asset(unit) => { unit.update(); }
                Unit::Task(unit) => { unit.update(); }
                Unit::User(unit) => { unit.update(); }
            }
            Some(unit)
        }
        None => { None }
    }
}

pub fn delete(state: &mut HashMap<String, Unit>, key: &str) -> Option<Unit> {
    state.remove(key)
}
