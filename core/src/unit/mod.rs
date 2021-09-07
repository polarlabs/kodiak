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

pub fn factory(unit_type: UnitType, name: &str) -> Box<dyn CRUD> {
    match unit_type {
        UnitType::Asset => Box::new(Asset::create(name)),
        UnitType::Task => Box::new(Task::create(name)),
        UnitType::User => Box::new(User::create(name)),
    }
}
