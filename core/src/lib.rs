pub mod io;
pub mod unit;

use unit::{Unit, UnitType, Asset, Task, User, CRUD};

use std::collections::HashMap;

pub fn create(state: &mut HashMap<String, Unit>, unit_type: UnitType, name: &str) -> Option<Unit> {

    match unit_type {
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

pub fn read<'a>(state: &'a HashMap<String, Unit>, key: &str) -> Option<&'a Unit> {
    state.get(key)
}

pub fn read_by_unittype<'a>(state: &'a HashMap<String, Unit>, unittype: UnitType, key: &str) -> Option<&'a Unit> {

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
