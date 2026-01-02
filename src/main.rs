mod variables {
    pub mod numbers;
    pub mod characters_and_strings;
}

mod calculator {
    pub mod calculator;
}

mod user {
    pub mod user;
}

use variables::numbers::NumbersAndDecimals;
use variables::characters_and_strings::{ character, strings };
use calculator::calculator::{ add, sub, mul };
use user::user::User;
use crate::user::user::Presentable;

fn main() {
    println!("This is my first Rust program!");

    //numbers();
    NumbersAndDecimals::integers();
    NumbersAndDecimals::decimals();

    // strings
    character();
    strings();

    // calculators
    add(1, 2);
    sub(3, 2);
    mul(1, 2);

    // user
    let user1 = User::new_user("Jose S Reyes".to_string(), 28);
    println!("{:?}", user1);
    user1.present()
}

fn numbers() {
    // integer
    let age: i32 = 30;
    let temperature: i32 = -5;
    let quantity: i32 = 50;

    // decimal
    let price: f64 = 99.99;
    let pi: f64 = std::f64::consts::PI;

    println!(
        "\nAge: {}, Temperature: {}, Quantity: {}",
        age, temperature, quantity
    );
    println!("Price: {}, Pi: {}", price, pi);
}
