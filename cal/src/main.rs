use std::io::{self, Write};

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn subtract(a: f32, b: f32) -> f32 {
    a - b
}

fn multiply(a: f32, b: f32) -> f32 {
    a * b
}

fn divide(a: f32, b: f32) -> f32 {
    a / b
}

fn read_number(prompt: &str) -> f32 {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Ensure prompt is displayed immediately
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn main() {
    loop {
        let mut task = String::new();
        println!("Enter task (add, sub, mul, div) or 'exit' to quit:");
        io::stdin().read_line(&mut task).expect("Failed to read line");
        let task = task.trim();

        if task == "exit" {
            println!("Exiting the calculator. Goodbye!");
            break;
        }

        let num1 = read_number("Enter Number 1: ");
        let num2 = read_number("Enter Number 2: ");

        match task {
            "add" => println!("Result: {}", add(num1, num2)),
            "sub" => println!("Result: {}", subtract(num1, num2)),
            "mul" => println!("Result: {}", multiply(num1, num2)),
            "div" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                } else {
                    println!("Result: {}", divide(num1, num2));
                }
            },
            _ => println!("Sorry, invalid operation. Please try again."),
        }
    }
}