use super::super::traits::crud::CRUD;
use super::_unit::Unit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    email: Option<String>,
    unit: Unit,
}

impl User {
    pub fn new(email: Option<&str>) -> Self {
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
    fn create(&mut self, email: &str) {
        println!("Create on {:?}", self);
        self.email = Some(email.to_string());
    }

    fn key(&self) -> String {
        self.unit.key()
    }
}
