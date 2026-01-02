pub struct NumbersAndDecimals;

impl NumbersAndDecimals {
    // integer
    pub fn integers() {
        let age: i32 = 30;
        let temperature: i32 = -5;
        let quantity: i32 = 50;

        println!(
            "\nAge: {}, Temperature: {}, Quantity: {}",
            age, temperature, quantity
        );
    }

    // decimals
    pub fn decimals() {
        let price: f64 = 99.99;
        let pi: f64 = std::f64::consts::PI;
        println!("Price: {}, Pi: {}", price, pi);
    }
}