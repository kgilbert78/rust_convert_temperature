use std::io;

fn main() {
    println!("Welcome to the Temperature Conversion App! Would you like to:");
    println!("(1) convert from Fahrenheit to Celsius");
    println!("(2) convert from Celsius to Fahrenheit");

    let mut selection = String::new();
    let mut convert_from = "Fahrenheit";
    let mut convert_to = "Celcius";

    io::stdin().read_line(&mut selection).expect("failed to read line");

    if selection == "2" {
        convert_from = "Celcius";
        convert_to = "Fahrenheit";
    }

    println!("Please enter a temperature to convert from {} to {}:", convert_from, convert_to);

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).expect("failed to read line");

    // try shadowing later
    let temperature_float: f32 = temperature.trim().parse().unwrap(); // calling parse on an error value here somehow
    // trim to remove the newline character from stdin!

    // try switch statement later
    if convert_to == "Celcius" {
        fahrenheit_to_celsius(temperature_float);
    } else if convert_to == "Fahrenheit" {
        celsius_to_fahrenheit(temperature_float);
    } else {
        println!("Error")
    }

    // make this work above conditional function call
    io::stdin().read_line(&mut temperature).expect("failed to read line");
    // need error handling for bad input and loop with match to continue prompting on error
}
// Convert temperatures between Fahrenheit and Celsius. 
// Formulas are C = (F - 32) * 5/9 and F = (C * 9/5) + 32.
fn fahrenheit_to_celsius(input: f32) -> f32{
    let minus_32 = input - 32.0;
    let result = minus_32 * 0.5555555556;
    println!("{result}"); // remove here and assign function to variable to print result
    result
}

fn celsius_to_fahrenheit(input: f32) -> f32 {
    let nine_fifths = input * 0.5555555556;
    let result = nine_fifths + 32.0;
    println!("{result}"); // remove here and assign function to variable to print result
    result
}