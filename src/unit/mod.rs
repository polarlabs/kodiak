mod structs;
mod traits;

// Re-exporting all public structures
pub use structs::asset::Asset;
pub use structs::task::Task;
pub use structs::user::User;

pub use traits::crud::CRUD;

pub enum UnitType {
    Asset,
    Task,
    User,
}

pub fn factory(unit_type: UnitType) -> Box<dyn CRUD> {
    match unit_type {
        UnitType::Asset => Box::new(Asset::new(None)),
        UnitType::Task => Box::new(Task::new(None, None)),
        UnitType::User => Box::new(User::new(None)),
    }
}
