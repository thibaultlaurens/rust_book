use std::io;

fn main() {
    println!("Fahrenheit to Celsius converter");

    loop {
        println!("Please input your Fahrenheit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        let fahrenheit = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("Celsius: {}", celsius);
    }
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> f64 {
    let fahrenheit = fahrenheit as f64;
    (fahrenheit - 32.0) * (5.0 / 9.0)
}
