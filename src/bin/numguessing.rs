
use rand::Rng;
use std::io;

fn main() {
    let secret = rand::rng().random_range(1..=100);

    println!("Guess the number between 1 and 100!");

    loop {
        println!("Enter your guess:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input failed");

        let guess: u32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if guess < secret {
            println!("Too low!");
        } else if guess > secret {
            println!("Too high!");
        } else {
            println!("Correct! You win!");
            break;
        }
    }
}
