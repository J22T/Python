use std::io;

fn welcome() {
    println!("Welcome to calculator");
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

impl Op {
    fn from_str(s: &str) -> Result<Op, String> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            "**" => Ok(Op::Pow),
            "%" => Ok(Op::Mod),
            _ => Err("Invalid operation".to_string()),
        }
    }

    fn apply(&self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
            Op::Pow => a.powf(b),
            Op::Mod => a % b,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Pow => "**",
            Op::Mod => "%",
        }
    }
}

fn calculate() -> Result<(), String> {
   println!("Please type in the math operation you would like to complete:
+ for addition
- for subtraction
* for multiplication
/ for division
** for power
% for modulo");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).map_err(|e| e.to_string())?;

    // Rust has explicit types, you need to convert the string to a number with the parse function
    let operation = Op::from_str(operation.trim_end())?;
    let mut number_1 = String::new();
    let mut number_2 = String::new();

    println!("Please type in the first number");
    io::stdin().read_line(&mut number_1).map_err(|e| e.to_string())?;

    println!("Please type in the second number");
    io::stdin().read_line(&mut number_2).map_err(|e| e.to_string())?;

    let number_1 = number_1.trim_end().parse::<f64>().map_err(|e| e.to_string())?;
    let number_2 = number_2.trim_end().parse::<f64>().map_err(|e| e.to_string())?;

    println!("{} {} {} = {}", number_1, operation.to_str(), number_2, operation.apply(number_1,number_2));

    again()?;

    Ok(())
}

fn again() -> Result<(), String> {
    println!("Do you want to calculate again? Type y for yes or n for no");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).map_err(|e| e.to_string())?;

    if answer.trim_end().to_uppercase() == "Y" {
        calculate()?;
    } else if answer.trim_end().to_uppercase() == "N" {
        println!("Goodbye");
    } else {
        println!("Invalid answer");
        again()?;
    }
    Ok(())
}

fn main() {
    welcome();
    calculate().unwrap();
}

// Really cool studying over this; still understand zero of the above.