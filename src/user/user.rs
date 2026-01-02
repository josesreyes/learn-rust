#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
}

impl User {
    // constructors
    fn new_user1(name: String, age: u32) -> User {
        User { name, age }
    }

    pub fn new_user(name: String, age: u32) -> Self {
        Self { name, age }
    }

    // methods
    fn greeting(&self) {
        println!("\nHello, I'm {} and {} years old", self.name, self.age);
    }

}

pub trait Presentable { // public interface
    fn present(&self);
}

impl Presentable for User { // implement interface
    fn present(&self) {
        println!("\nI'm {} and {} years old", self.name, self.age);
    }
}