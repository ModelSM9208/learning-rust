use std::io::{self, Write};

fn get_positive_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(val) if val >= 0.0 => return val,
            Ok(_) => println!("Value must be greater than or equal to zero."),
            Err(_) => println!("That's not a number. Try again."),
        }
    }
}

fn calculate_tax_and_total(cost: f64, tax_rate: f64) -> (f64, f64) {
    let tax = cost * (tax_rate/100.0);
    let total = cost + tax;
    (tax, total)
}

fn main() {
    let cost = get_positive_number("Enter pre tax cost: ");
    let tax_rate = get_positive_number("Enter tax as a percent example 10 would be 10% tax rate: ");
    let (tax_paid, total) = calculate_tax_and_total(cost, tax_rate);
    println!("sub total: ${:.2} plus tax: ${:.02} Total cost: ${:.2}", cost, tax_paid, total);
}