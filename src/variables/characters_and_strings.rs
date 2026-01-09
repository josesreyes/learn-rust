pub fn character() {
    let initial: char = '?';
    println!("\ninitial = '{}'", initial);
}

pub fn strings() {
    let example: &str = "hello"; // más liviano, inmutable, ideal para pasar referencias.
    let example2: String = "World".to_string(); // dueño de la memoria, mutable, ideal para construir y modificar texto.
    let mut message = String::from("Hello"); // mutable
    message.push_str(", Joss!");

    println!("\nexample = '{}'", example);
    println!("example2 = '{}'", example2);
}
