
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

const FILE_NAME: &str = "expenses.csv";

struct Expense {
    name: String,
    amount: f64,
}

fn main() {
    let mut expenses = load_expenses();

    loop {
        println!("\n=== EXPENSE TRACKER ===");
        println!("1. Add expense");
        println!("2. View expenses");
        println!("3. View total");
        println!("4. Exit");
        println!("Choose an option:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Input failed");

        match choice.trim() {
            "1" => {
                if let Some(expense) = add_expense() {
                    save_expense(&expense);
                    expenses.push(expense);
                }
            }
            "2" => view_expenses(&expenses),
            "3" => view_total(&expenses),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Try again."),
        }
    }
}

fn add_expense() -> Option<Expense> {
    println!("Enter expense name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).ok()?;
    let name = name.trim();

    if name.is_empty() || name.contains(',') || name.contains('\n') {
        println!("Enter a non-empty name without commas.");
        return None;
    }

    println!("Enter amount:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    let amount = match input.trim().parse::<f64>() {
        Ok(value) if value.is_finite() && value > 0.0 => value,
        _ => {
            println!("Enter a valid positive amount.");
            return None;
        }
    };

    println!("Expense added!");
    Some(Expense {
        name: name.to_string(),
        amount,
    })
}

fn view_expenses(expenses: &[Expense]) {
    if expenses.is_empty() {
        println!("No expenses recorded.");
        return;
    }

    for (i, expense) in expenses.iter().enumerate() {
        println!("{}. {} - {:.2}", i + 1, expense.name, expense.amount);
    }
}

fn view_total(expenses: &[Expense]) {
    let total: f64 = expenses.iter().map(|e| e.amount).sum();
    println!("Total expenses: {:.2}", total);
}

fn save_expense(expense: &Expense) {
    let result = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_NAME);

    match result {
        Ok(mut file) => {
            if let Err(error) = writeln!(file, "{},{:.2}", expense.name, expense.amount) {
                println!("Failed to save expense: {error}");
            }
        }
        Err(error) => println!("Failed to open file: {error}"),
    }
}

fn load_expenses() -> Vec<Expense> {
    let contents = match fs::read_to_string(FILE_NAME) {
        Ok(contents) => contents,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Vec::new();
        }
        Err(error) => {
            println!("Failed to load expenses: {error}");
            return Vec::new();
        }
    };

    contents
        .lines()
        .filter_map(|line| {
            let (name, amount) = line.rsplit_once(',')?;
            let amount = amount.parse::<f64>().ok()?;

            if name.is_empty() || !amount.is_finite() || amount <= 0.0 {
                return None;
            }

            Some(Expense {
                name: name.to_string(),
                amount,
            })
        })
        .collect()
}
