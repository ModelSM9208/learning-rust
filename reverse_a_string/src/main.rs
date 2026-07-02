use std::io::{self, Write};

fn main() {
    print!("Enter a string to reverse: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let reversed: String = input.trim().chars().rev().collect();

    println!("Reversed: {reversed}");
}
