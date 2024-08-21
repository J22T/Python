use std::io;

fn welcome() {
    println!("Welcome to calculator");
}

fn calculate() {
   println!("Please type in the math operation you would like to complete:
+ for addition
- for subtraction
* for multiplication
/ for division
** for power
% for modulo");

    let mut operation = String::new();

    io::stdin().read_line(&mut operation).expect("Failed to read line");

    // Rust has explicit types, you need to convert the string to a number with the parse function
    let operation = operation.trim_end();
    
    let mut number_1 = String::new();
    let mut number_2 = String::new();

    println!("Please type in the first number");
    io::stdin().read_line(&mut number_1).expect("Failed to read line");

    println!("Please type in the second number");
    io::stdin().read_line(&mut number_2).expect("Failed to read line");

    let number_1 = number_1.trim_end().parse::<f64>().unwrap();
    let number_2 = number_2.trim_end().parse::<f64>().unwrap();

    if operation == "+" {
        println!("The result is: {}", number_1 + number_2);
    } else if operation == "-" {
        println!("The result is: {}", number_1 - number_2);
    } else if operation == "*" {
        println!("The result is: {}", number_1 * number_2);
    } else if operation == "/" {
        println!("The result is: {}", number_1 / number_2);
    } else if operation == "**" {
        println!("The result is: {}", number_1.powf(number_2));
    } else if operation == "%" {
        println!("The result is: {}", number_1 % number_2);
    } else {
        println!("Invalid operation");
    }

    again();
}

fn again() {
    println!("Do you want to calculate again? Type y for yes or n for no");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");

    if answer.trim_end().to_uppercase() == "Y" {
        calculate();
    } else if answer.trim_end().to_uppercase() == "N" {
        println!("Goodbye");
    } else {
        println!("Invalid answer");
        again();
    }
}

fn main() {
    welcome();
    calculate();
}

// Wow