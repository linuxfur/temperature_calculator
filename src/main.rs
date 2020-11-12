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
    use std::io;

    let mut temperature = String::new();
    println!("Enter a number:\n");
    io::stdin().read_line(&mut temperature).expect("Failed to read input.");

    let temperature: f64 = temperature.trim().parse().expect("Please enter a number.");
    
    println!("Celsius: {}°", celsius(temperature));
    println!("Fahrenheit: {}°", fahrenheit(temperature));
    println!("Kelvin: {}°", kelvin(temperature));
}