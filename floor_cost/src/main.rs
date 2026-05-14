use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("Enter width: ");
    let mut width_input = String::new();
    io::stdin().read_line(&mut width_input).expect("Failed to read line");

    println!("Enter length: ");
    let mut length_input = String::new();
    io::stdin().read_line(&mut length_input).expect("Failed to read line");

    println!("Enter cost per unit: ");
    let mut cost_per_unit_input = String::new();
    io::stdin().read_line(&mut cost_per_unit_input).expect("Failed to read line");

    let width: f64 = width_input.trim().parse().expect("Please enter a number");
    println!("{}", width);
    let length: f64 = length_input.trim().parse().expect("Please enter a number");
    println!("{}", length);
    let cost_per_unit: f64 = cost_per_unit_input.trim().parse().expect("cash money only");
    println!("{}", cost_per_unit);
    let total_cost = width * length * cost_per_unit;
    println!("{}", total_cost)
}
