use std::fmt::{Debug, Formatter};

#[typetag::serde(tag = "type")]
pub trait CRUD {
    fn create(identifier: &str) -> Self
        where Self: Sized;

    fn read(&self) {}

    fn update(&mut self);

    fn delete(&self) {}

    fn key(&self) -> String;
}

impl Debug for dyn CRUD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo: implement a more powerful debug for all CRUD trait objects
            f.debug_struct("dyn CRUD")
                .field("key", &self.key())
                .finish()
    }
}
