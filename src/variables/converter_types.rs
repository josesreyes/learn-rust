pub fn converter_types() {
    // Integer to Float conversion
    let number: i32 = 5;
    let decimal: f64 = number as f64; // becomes 5.0

    // Float to Integer conversion
    let decimal: f64 = 9.7;
    let number: i32 = decimal as i32; // becomes 9 (decimal part is truncated)

    // Declare and initialize variables
    let price: f64 = 99.99;
    let int_price: i32 = price as i32;

    // Output the values
    println!("\nCasting Variables");
    println!("Price: {}", price);
    println!("Int Price: {}", int_price);
}