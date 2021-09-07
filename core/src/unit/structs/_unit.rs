use chrono::{DateTime, Utc};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    key: String,
    state: UnitState,
    created: DateTime<Utc>,
    updated: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
enum UnitState {
    On,
    Off,
}

impl Unit {
    pub fn new() -> Self {
        Unit {
            key: Uuid::new_v4().to_string(),
            state: UnitState::On,
            created: Utc::now(),
            updated: None,
        }
    }

    pub fn update(&mut self) {
        self.updated = Some(Utc::now());
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }
}
