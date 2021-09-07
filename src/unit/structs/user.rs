use super::super::traits::crud::CRUD;
use super::_unit::Unit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    email: Option<String>,
    unit: Unit,
}

impl User {
    fn new(email: Option<&str>) -> Self {
        User {
            email: match email {
                Some(email) => Some(email.to_string()),
                None => None,
            },
            unit: Unit::new(),
        }
    }
}

#[typetag::serde]
impl CRUD for User {
    fn create(email: &str) -> Self {
        User::new(Some(email))
    }

    fn update(&mut self) {
        self.unit.update();
    }

    fn key(&self) -> String {
        self.unit.key()
    }
}
