pub fn mutable_variables() {
    let x = 5; // x is immutable
    let mut y= 5; // y is mutable

    println!("\nImmutable variables");
    println!("x = {} and y = {}", x, y);

    print!("Mutable variables");
    y = 6; // value is reasignable
    println!("x = {} and y = {}", x, y);
}