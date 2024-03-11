use std::io::{self, Write};

enum Token {
    //Expression(Vec<Token>),
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

fn tokenize(input: String) -> Result<Vec<Token>, &'static str> {
    let mut tokens: Vec<Token> = Vec::new();    
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
                tokens.push(Token::Value(num));
                slice_start = None;
            }
        match c {
                '^' => tokens.push(Token::Operation(Operator::Exponent)),
                '*' => tokens.push(Token::Operation(Operator::Multiplication)),
                '/' => tokens.push(Token::Operation(Operator::Division)),
                '+' => tokens.push(Token::Operation(Operator::Addition)),
                '-' => tokens.push(Token::Operation(Operator::Subtraction)),
                _ => return Result::Err("Invalid input"),
            }   
    }

    if let Some(start) = slice_start {
        let num = parse_to_f64(start, input.len())?;
        tokens.push(Token::Value(num));
    }

    Ok(tokens)
}

fn evaluate_expression(mut tokens: Vec<Token>) -> Result<f64, &'static str> {
    loop {
        
        let mut priority = 0;
        let mut op_i = None;

        {
            let mut i = 0;
            for token in &tokens {
                if let Token::Operation(op) = token {
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
                Token::Operation(_) => return Err("Invalid expression"),
                Token::Value(n) => return Ok(n),
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
            Token::Operation(op) => {
                let left = match tokens[op_i - 1] {
                    Token::Operation(_) => return Err("The left token of the operation is another operation"),
                    Token::Value(n) => n,
                };
                
                let right = match tokens[op_i+1] {
                    Token::Operation(_) => return Err("The right token of the operation is another operation"),
                    Token::Value(n) => n,
                };

                let result = match op {
                    Operator::Exponent => left.powf(right),
                    Operator::Multiplication => left*right,
                    Operator::Division => left/right,
                    Operator::Addition => left+right,
                    Operator::Subtraction => left-right,
                };

                tokens.splice(op_i-1..=op_i+1, vec![Token::Value(result)]);
            },
            Token::Value(_) => return Err("Indexed operator is a number"),
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