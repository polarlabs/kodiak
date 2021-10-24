use std::str::FromStr;
use crate::{Asset, Task, User};

use serde::{Deserialize, Serialize};

// Aliases used in path of REST API endpoints
#[derive(Debug, Deserialize, PartialEq)]
pub enum UnitType {
    #[serde(alias = "units")]
    Unit,
    #[serde(alias = "assets")]
    Asset,
    #[serde(alias = "tasks")]
    Task,
    #[serde(alias = "users")]
    User,
}

// Used in CLI
impl FromStr for UnitType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unit" => Ok(UnitType::Unit),
            "asset" => Ok(UnitType::Asset),
            "task" => Ok(UnitType::Task),
            "user" => Ok(UnitType::User),
            _ => { Err(()) }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Unit {
    Asset(Asset),
    Task(Task),
    User(User),
}

impl Unit {
    pub fn is_a(&self, unittype: &UnitType) -> bool {
        match self {
            Unit::Asset(_) => unittype == &UnitType::Asset,
            Unit::Task(_) => unittype == &UnitType::Task,
            Unit::User(_) => unittype == &UnitType::User,
        }
    }
}
