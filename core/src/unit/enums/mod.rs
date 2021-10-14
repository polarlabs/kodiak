use crate::{Asset, Task, User};

use serde::{Deserialize, Serialize};

#[derive(PartialEq)]
pub enum UnitType {
    Asset,
    Task,
    User,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Unit {
    Asset(Asset),
    Task(Task),
    User(User),
}
