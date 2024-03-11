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

fn evaluate_expression(mut tokens: Vec<Expression>) -> Result<f64, &'static str> {
    loop {
        
        let mut priority = 0;
        let mut op_i = None;

        {
            let mut i = 0;
            for token in &tokens {
                if let Expression::Operation(op) = token {
                    match op {
                        Operator::Exponent => {
                            if priority < 2 {
                                priority = 2;
                                op_i = Some(i);
                            }
                        },
                        Operator::Multiplication => {
                            if priority < 1 {
                                priority = 1;
                                op_i = Some(i);
                            }
                        },
                        Operator::Division => {
                            if priority < 1 {
                                priority = 1;
                                op_i = Some(i);
                            }
                        },
                        Operator::Addition => {
                            if priority == 0 && op_i.is_none() {
                                op_i = Some(i);
                            }
                        },
                        Operator::Subtraction => {
                            if priority == 0 && op_i.is_none() {
                                op_i = Some(i);
                            }
                        },
                    }
                }
                i += 1;
            }
        }

        if tokens.len() == 1 {
            match tokens[0] {
                Expression::Operation(_) => return Err("Invalid expression"),
                Expression::Value(n) => return Ok(n),
            }    
        }
    
        if tokens.len() < 3 {
            return Err("invalid amount of tokens remaining");
        }
       
        let op_i = match op_i {
            Some(n) => n,
            None => return Err("No operation found for calculation"),
        };
        
        match &tokens[op_i] {
            Expression::Operation(op) => {
                let left = match tokens[op_i - 1] {
                    Expression::Operation(_) => return Err("The left token of the operation is another operation"),
                    Expression::Value(n) => n,
                };
                
                let right = match tokens[op_i+1] {
                    Expression::Operation(_) => return Err("The right token of the operation is another operation"),
                    Expression::Value(n) => n,
                };

                let result = match op {
                    Operator::Exponent => left.powf(right),
                    Operator::Multiplication => left*right,
                    Operator::Division => left/right,
                    Operator::Addition => left+right,
                    Operator::Subtraction => left-right,
                };

                tokens.splice(op_i-1..=op_i+1, vec![Expression::Value(result)]);
            },
            Expression::Value(_) => return Err("Indexed operator is a number"),
        };

    }
}

fn main() {
    let mut input = String::new();

    // Raggedy tests
    let test_tokens = tokenize(String::from("9-4/2^2+1")).unwrap();
    let test_result = evaluate_expression(test_tokens).unwrap();
    assert_eq!(test_result, 9.0);

    print!("Enter your equation: ");
    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let tokens = match tokenize(input.trim().to_string()) {
        Ok(vec) => vec,
        Err(e) => {
            println!("{}",e);
            panic!("Tokenization failed");
        },
    };

    let result = match evaluate_expression(tokens) {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            panic!("Calculation failed");
        },
    };

    println!("The result is: {}", result);   
}