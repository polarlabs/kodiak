use crate::CRUD;
use super::_unit::Unit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    subject: Option<String>,
    status: Option<String>,
    unit: Unit,
}

impl Task {
    fn new(subject: Option<&str>, status: Option<&str>) -> Self {
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

impl CRUD for Task {
    fn create(subject: &str) -> Self {
        Task::new(Some(subject), None)
    }

    fn update(&mut self) -> &Self {
        self.unit.update();
        self
    }

    fn key(&self) -> String {
        self.unit.key()
    }
}
