use core::slice;
use std::io::{self, Write};

enum Equation {
    SubEquation,
    Operation,
    Value(f64)
}


fn tokenize(input: String) -> Result<Vec<Equation>, &'static str> {

    let mut tokens: Vec<Equation> = Vec::new();    
    //let mut parenthesis_count = None;
    let mut slice_start = None; 

    for (i, c) in input.chars().enumerate() {
        
        if c.is_numeric() || c == '.' {

            if slice_start == None {
                slice_start = Some(i);
            }
            continue;

        } else {

            match slice_start {
                Some(start) => {
                    let slice = &input[start..i];
                    let num: f64 = slice.parse().expect("Parsing input to f64");
                    slice_start = None;

                    tokens.push(Equation::Value(num));
                },
                None => (),
            }
        }     
    }

    match slice_start {
        Some(start) => {
            let slice = &input[start..];
            let num: f64 = slice.parse().expect("Parsing input to f64");

            tokens.push(Equation::Value(num));
        },
        None => (),
    }



    if tokens.len() > 0 {
        return  Ok(tokens);
    } else {
        return Err("Nothing to tokenized");
    }
}

fn main() {
    let mut input = String::new();

    print!("Please enter your equation: ");
    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let test = tokenize(String::from("23"));
    let test2 = tokenize(String::from(".97618200"));


    match test {
        Ok(token) => println!("{}", token.len()),
        Err(e) => println!("{}", e),
    }

    //let tokens = tokenize(input);
}
