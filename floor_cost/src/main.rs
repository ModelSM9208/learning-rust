use std::io::{self, Write};

fn get_positive_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(val) if val > 0.0 => return val,
            _ => println!("Invalid input. Please enter a positive number."),
        }
    }
}

fn main() {
    let width = get_positive_number("Enter width (in feet): ");
    let length = get_positive_number("Enter length (in feet): ");
    let cost = get_positive_number("Enter cost per unit: ");
    println!("Total cost: ${:.2}", width * length * cost);
}
