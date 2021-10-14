use crate::CRUD;
use super::_unit::Unit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

impl CRUD for User {
    fn create(email: &str) -> Self {
        User::new(Some(email))
    }

    fn update(&mut self) -> &Self {
        self.unit.update();
        self
    }

    fn key(&self) -> String {
        self.unit.key()
    }
}
