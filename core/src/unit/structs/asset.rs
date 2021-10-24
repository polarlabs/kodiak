use crate::CRUD;
use super::_unit::Unit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asset {
    name: Option<String>,

    #[serde(flatten)]
    unit: Unit,
}

impl Asset {
    fn new(name: Option<&str>) -> Self {
        Asset {
            name: match name {
                Some(name) => Some(name.to_string()),
                None => None,
            },
            unit: Unit::new(),
        }
    }
}

impl CRUD for Asset {
    fn create(name: &str) -> Self {
        Asset::new(Some(name))
    }

    fn update(&mut self) -> &Self {
        self.unit.update();
        self
    }

    fn key(&self) -> String {
        self.unit.key()
    }
}
