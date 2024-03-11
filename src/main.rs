use std::io::{self, Write};

enum Expression {
    Operation(Operator),
    Value(f64)
}

enum Operator {
    Exponent,
    Multiplication,
    Division,
    Addition,
    Subtraction
}

fn tokenize(input: String) -> Result<Vec<Expression>, &'static str> {
    let mut tokens: Vec<Expression> = Vec::new();    
    let mut slice_start = None; 
    
    let parse_to_f64 = |start: usize, end: usize| -> Result<f64, &str> {
        let slice = &input[start..end];
        let num = slice.parse::<f64>();

        match num {
            Ok(n) => Ok(n),
            Err(_) => Err("failed parsing input to f64"),
        }
    };   

    for (i, c) in input.chars().enumerate() {
        if c.is_numeric() || c == '.' {
            if slice_start.is_none() {
                slice_start = Some(i);
            } 
            continue;
        }

        if let Some(start) = slice_start {
                let num = parse_to_f64(start, i)?;
                tokens.push(Expression::Value(num));
                slice_start = None;
            }
        match c {
                '^' => tokens.push(Expression::Operation(Operator::Exponent)),
                '*' => tokens.push(Expression::Operation(Operator::Multiplication)),
                '/' => tokens.push(Expression::Operation(Operator::Division)),
                '+' => tokens.push(Expression::Operation(Operator::Addition)),
                '-' => tokens.push(Expression::Operation(Operator::Subtraction)),
                _ => return Result::Err("Invalid input"),
            }   
    }

    if let Some(start) = slice_start {
        let num = parse_to_f64(start, input.len())?;
        tokens.push(Expression::Value(num));
    }

    Ok(tokens)
}

fn main() {
    let mut input = String::new();

    print!("Enter your equation: ");
    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let tokens = tokenize(input.trim().to_string());

    match tokens {
        Ok(list) => {
            for element in list {
                match element {
                    Expression::Value(num) => println!("{}", num),
                    Expression::Operation(operator) => {
                        match operator {
                            Operator::Exponent => println!("^"),
                            Operator::Multiplication => println!("*"),
                            Operator::Addition => println!("+"),
                            Operator::Division => println!("/"),
                            Operator::Subtraction => println!("-"),
                        }
                    }
                }
            }
        },
        Err(e) => println!("{}", e),
    }
}