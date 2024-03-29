use std::io;

fn main() {
    println!("Welcome to the Temperature Conversion App! Would you like to:");
    println!("(1) convert from Fahrenheit to Celsius");
    println!("(2) convert from Celsius to Fahrenheit");
    println!("Enter 1 or 2:");

    let mut selection = String::new();
    let mut convert_from = "Fahrenheit";
    let mut convert_to = "Celcius";

    io::stdin().read_line(&mut selection).expect("failed to read line");

    if selection.trim() == "2" {
        convert_from = "Celcius";
        convert_to = "Fahrenheit";
    }

    println!("Please enter a temperature to convert from {} to {}:", convert_from, convert_to);

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).expect("failed to read line");
    let temperature: f32 = temperature.trim().parse().unwrap(); // shadowing
    // trim to remove the newline character from stdin!
    
    let converted_temperature = convert(temperature, convert_to);
    
    println!("{} {}", converted_temperature, convert_to);
    // need error handling for bad input and loop with match to continue prompting on error
}

// Convert temperatures between Fahrenheit and Celsius. 
// Formulas are C = (F - 32) * 5/9 and F = (C * 9/5) + 32.
fn convert(input: f32, output_type: &str) -> f32 {
    let mut result = input;
    if output_type == "Celcius" {
        let minus_32 = input - 32.0;
        result = minus_32 * 0.5555555556;
    } else if output_type == "Fahrenheit" {
        let nine_fifths = input * 0.5555555556;
        result = nine_fifths + 32.0;
    }
    result
}