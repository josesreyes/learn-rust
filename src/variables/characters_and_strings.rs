pub fn character() {
    let initial: char = '?';
    println!("\ninitial = '{}'", initial);
}

pub fn strings() {
    let example: &str = "hello";
    let example2: String = "World".to_string();
    let mut message = String::from("Hello");
    message.push_str(", Joss!");

    println!("\nexample = '{}'", example);
    println!("example2 = '{}'", example2);
}
