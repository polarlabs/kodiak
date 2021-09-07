pub struct Path {
    prefix: String,
}

impl Path {
    pub fn new(prefix: &str) -> Path {
        Path {
            prefix: prefix.to_string(),
        }
    }

    pub fn sub(&self, sub: &str) -> String {
        self.prefix.to_owned() + sub
    }
}
