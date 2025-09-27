fn main() {
    println!("Hello, world!");
}
// Assignment 1: Temperature Converter
// Converts between Fahrenheit and Celsius.

const FREEZING_F: f64 = 32.0;

// Converts Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_F) * 5.0 / 9.0
}

// Converts Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_F
}

fn main() {
    let mut f: f64 = 32.0;
    let c = fahrenheit_to_celsius(f);
    println!("{f}°F = {c:.1}°C");

    for i in 1..=5 {
        let next_f = f + i as f64;
        let next_c = fahrenheit_to_celsius(next_f);
        println!("{next_f}°F = {next_c:.1}°C");
    }
}
