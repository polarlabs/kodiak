use super::super::traits::crud::CRUD;
use super::_unit::Unit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    name: Option<String>,
    unit: Unit,
}

impl Asset {
    pub fn new(name: Option<&str>) -> Self {
        Asset {
            name: match name {
                Some(name) => Some(name.to_string()),
                None => None,
            },
            unit: Unit::new(),
        }
    }
}

#[typetag::serde]
impl CRUD for Asset {
    fn create(&mut self, name: &str) {
        println!("Create on {:?}", self);
        self.name = Some(name.to_string());
        println!("Create on {:?}", self);
    }

    fn key(&self) -> String {
        self.unit.key()
    }
}
