struct User {
    name: String,
    age: u32,
}

impl User {
    // constructors
    fn new_user1(name: String, age: u32) -> User {
        User { name, age }
    }

    fn new_user(name: String, age: u32) -> Self {
        Self { name, age }
    }

    // methods
    fn greeting(&self) {
        println!("Hello, I'm {} and {} years old", self.nombre, self.edad);
    }

}

trait Presentable { // interface
    fn present(&self);
}

impl Presentable for User { // implement interface
    fn present(&self) {
        println!("I'm {} and {} years old", self.name, self.age);
    }
}