use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Unit {
    key: String,
    state: UnitState,
    created: DateTime<Utc>,
    updated: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
