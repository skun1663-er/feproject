
use std::io;

fn main() {

    println!("\n=== Rust Calculator ===");

    let first = read_number("Enter the first number: ");
    let operator = read_operator();
    let second = read_number("Enter the second number: ");

    let result = match operator.as_str() { //chooses the appropriate arithmetic operation
        "+" => Some(first + second),
        "-" => Some(first - second),
        "*" => Some(first * second),
        "/" if second != 0.0 => Some(first / second),
        "/" => {
            println!("Error: Cannot divide by zero!");
            None
        }
        _ => {
            println!("Error: Unknown operator!");
            None
        }
    };

    if let Some(value) = result {
        println!("Result: {}", value);
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input) //reads text typed into the terminal
            .expect("Failed to read input");

        match input.trim().parse::<f64>() { //converts the input into a floating-point number, such as 10.5
            Ok(number) => return number,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn read_operator() -> String {
    println!("Enter operator (+, -, *, /): ");

    let mut operator = String::new();

    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read input");

    operator.trim().to_string()
}

