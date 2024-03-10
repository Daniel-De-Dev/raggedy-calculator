use std::io::{self, Write};

enum Expression {
    Operation(Operator),
    Value(f64)
}

enum Operator {
    Multiplication,
    Division,
    Addition
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
        if c.is_numeric() || c == '.' || c == '-' {
            if slice_start.is_none() {
                slice_start = Some(i);
            } else if c == '-' {
                if let Some(start) = slice_start {
                    let num = parse_to_f64(start, i)?;
                    tokens.push(Expression::Value(num));
                    tokens.push(Expression::Operation(Operator::Addition));
                }
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
                '*' => tokens.push(Expression::Operation(Operator::Multiplication)),
                '/' => tokens.push(Expression::Operation(Operator::Division)),
                '+' => tokens.push(Expression::Operation(Operator::Addition)),
                _ => return Result::Err("Invalid input provided"),
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
                            Operator::Multiplication => println!("*"),
                            Operator::Addition => println!("+"),
                            Operator::Division => println!("/"),
                        }
                    }
                }
            }
        },
        Err(e) => println!("{}", e),
    }
}