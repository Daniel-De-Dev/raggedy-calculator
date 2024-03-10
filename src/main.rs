use std::io::{self, Write};

enum Equation {
    SubEquation,
    Operation,
    Value(f64)
}

fn tokenize(input: String) -> Result<Vec<Equation>, &'static str> {
    let mut tokens: Vec<Equation> = Vec::new();    
    let mut slice_start = None; 

    let mut parse_and_push = |start: usize, end: usize| {
        let slice = &input[start..end];
        let num = slice.parse();

        match num {
            Ok(n) => { 
                tokens.push(Equation::Value(n));
                Ok(())
            },
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
            parse_and_push(start, i)?;
            slice_start = None;
        }   
    }

    if let Some(start) = slice_start {
        parse_and_push(start, input.len())?
    }

    Ok(tokens)
}

fn main() {
    let mut input = String::new();

    print!("Enter your equation: ");
    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let tokens = tokenize(input);

    match tokens {
        Ok(list) => {
            for element in list {
                match element {
                    Equation::Value(num) => println!("{}", num),
                    _ => (),
                }
            }
        },
        Err(e) => println!("{}", e),
    }
}