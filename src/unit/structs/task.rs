use super::super::CRUD;
use super::_unit::Unit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    subject: Option<String>,
    status: Option<String>,
    unit: Unit,
}

impl Task {
    pub fn new(subject: Option<&str>, status: Option<&str>) -> Self {
        Task {
            subject: match subject {
                Some(subject) => Some(subject.to_string()),
                None => None,
            },
            status: match status {
                Some(status) => Some(status.to_string()),
                None => None,
            },
            unit: Unit::new(),
        }
    }
}

#[typetag::serde]
impl CRUD for Task {
    fn create(&mut self, subject: &str) {
        println!("Create on {:?}", self);
        self.subject = Some(subject.to_string());
        println!("Create on {:?}", self);
    }

    fn key(&self) -> String {
        self.unit.key()
    }
}
