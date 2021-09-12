pub trait CRUD {
    fn create(identifier: &str) -> Self
        where Self: Sized;

    fn read(&self) {}

    fn update(&mut self) -> &Self;

    fn delete(&self) {}

    fn key(&self) -> String;
}
