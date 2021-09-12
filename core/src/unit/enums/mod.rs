use crate::{Asset, Task, User};

use serde::{Deserialize, Serialize};

pub enum UnitType {
    Asset,
    Task,
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Unit {
    Asset(Asset),
    Task(Task),
    User(User),
}
