use std::io;

// Step 1: Define the Operation enum with variants holding two f64 values
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Step 2 & 3: Implement the calculate function using pattern matching
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero.");
                std::f64::NAN // Return 'Not a Number' if dividing by zero
            }
        }
    }
}

fn main() {
    // Step 4: Prompt user for inputs
    println!("Enter the first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please type a number!");

    println!("Enter the operation (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let operator = operator.trim();

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please type a number!");

    // Step 5 & 6: Parse input and create an Operation enum instance
    let operation = match operator {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    // Step 7: Call calculate and Step 8: Print the result
    let result = calculate(operation);
    println!("The result is: {}", result);
}