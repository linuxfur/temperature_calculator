use std::io;
use std::env;

fn celsius(f: f64) -> f64 { // f being Fahrenheit
    let y = (f - 32.0) * 5.0/9.0;
    let formatted = (y * 100.0).round() / 100.0;
    return formatted;
}

fn fahrenheit(c: f64) -> f64 { // c being Celsius
    let y = (c * 9.0/5.0) + 32.0;
    let formatted = (y * 100.0).round() / 100.0;
    return formatted;
}

fn kelvin(c: f64) -> f64 { // c being Celsius
    let y = c + 273.15;
    let formatted = (y * 100.0).round() / 100.0;
    return formatted;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len >= 2 {
        let temperature = String::from(&args[1]);
        let temperature: f64 = temperature.trim().parse().expect("you shouldn't see this");
        println!("Celsius: {}°", celsius(temperature));
        println!("Fahrenheit: {}°", fahrenheit(temperature));
        println!("Kelvin: {}°", kelvin(temperature));
    }
    else {
        let mut temperature = String::new();
        println!("{:?}", args);
        println!("Enter a number:\n");
        io::stdin().read_line(&mut temperature).expect("Failed to read input.");
        
        let temperature: f64 = temperature.trim().parse().expect("Please enter a number.");
        
        println!("Celsius: {}°", celsius(temperature));
        println!("Fahrenheit: {}°", fahrenheit(temperature));
        println!("Kelvin: {}°", kelvin(temperature));
    }
}