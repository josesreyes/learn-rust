use learn_rust::calculator::calculator::{add, mul, sub};
use learn_rust::user::user::{Presentable, User};
use learn_rust::variables::booleans;
use learn_rust::variables::characters_and_strings::{character, strings};
use learn_rust::variables::converter_types::converter_types;
use learn_rust::variables::mutable_variables::mutable_variables;
use learn_rust::variables::numbers::NumbersAndDecimals;

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
    user1.present();

    // booleans
    booleans::booleans();

    // mutable_variables
    mutable_variables();

    // converter or casting
    converter_types();
}

/*fn numbers() {
    // integer
    let age: i32 = 30;
    let temperature: i32 = -5;
    let quantity: i32 = 50;

    // decimal
    let price: f64 = 99.99;    let pi: f64 = std::f64::consts::PI;

    println!(
        "\nAge: {}, Temperature: {}, Quantity: {}",
        age, temperature, quantity
    );
    println!("Price: {}, Pi: {}", price, pi);
}*/
