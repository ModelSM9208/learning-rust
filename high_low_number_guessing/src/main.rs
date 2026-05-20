use std::cmp::Ordering;
use std::io::{self, Write};

fn start_guessing(prompt: &str) {
    'session: loop {
        let secret = rand::random_range(1..=100);
//        println!("DEBUG secret: {secret}");
        loop {
            print!("{}", prompt);
            io::stdout().flush().expect("Failed to flush");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let guess: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not a number — jacking out. Thanks for playing.");
                    break 'session; // kill EVERYTHING
                }
            };
            match guess.cmp(&secret) {
                Ordering::Less => {
                    println!("Too low")
                }
                Ordering::Greater => {
                    println!("Too high")
                }
                Ordering::Equal => {
                    println!("Just right");
                    break;
                }
            }
        }
    }
}

fn main() {
    start_guessing("Please enter a number for your guess: ")
}
