pub struct Client {
    pub name: String,
}

impl Client {
    pub fn new(name: &str) -> Client {
        Client {
            name: name.to_string(),
        }
    }

    pub fn hello(&self) {
        println!("Hello!! I am {}", self.name);
    }
}
