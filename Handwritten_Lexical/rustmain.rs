use std::io::{self, Read};

#[derive(Debug)]
enum Token {
    Keyword(String),
    Identifier(String),
    Operator(String),
    Delimiter(String),
    Number(String),
    Preprocessor(String),
    Unknown(char),
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let keywords = vec![
        "int", "char", "double", "float", "main", "switch", "case", 
        "break", "if", "return", "default", "void"
    ];
    let operators = vec!["+", "-", "*", "/", "=", "==", "%", ">", "<"];
    let delimiters = vec!["(", ")", "{", "}", ";", ",", ":", "\"", "'"];

    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Handle Whitespace
            c if c.is_whitespace() => {
                chars.next();
            }
            // Handle Preprocessor Directives
            '#' => {
                let mut line = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc == '\n' { break; }
                    line.push(chars.next().unwrap());
                }
                println!("> PREPROCESSOR {}", line); [cite: 866-867]
            }
            // Handle Identifiers and Keywords
            c if c.is_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_alphanumeric() || nc == '_' {
                        ident.push(chars.next().unwrap());
                    } else { break; }
                }
                if keywords.contains(&ident.as_str()) {
                    println!("<KEYWORD {}>", ident); [cite: 868, 877]
                } else {
                    println!("<IDENTIFIER {}>", ident); [cite: 870, 880]
                }
            }
            // Handle Numbers
            c if c.is_digit(10) => {
                let mut num = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_digit(10) || nc == '.' {
                        num.push(chars.next().unwrap());
                    } else { break; }
                }
                println!("<NUMBER {}>", num); [cite: 928, 959]
            }
            // Handle Operators
            c if operators.contains(&c.to_string().as_str()) => {
                let mut op = c.to_string();
                chars.next();
                // Check for double-character operators like ==
                if let Some(&nc) = chars.peek() {
                    let combo = format!("{}{}", op, nc);
                    if combo == "==" {
                        op = combo;
                        chars.next();
                    }
                }
                println!("<OPERATOR {}>", op); [cite: 895, 924]
            }
            // Handle Delimiters
            c if delimiters.contains(&c.to_string().as_str()) => {
                println!("<DELIMITER {}>", c); [cite: 871, 881]
                chars.next();
            }
            // Handle Unknown Characters
            _ => {
                println!("<UNKNOWN {}>", c); [cite: 910, 938]
                chars.next();
            }
        }
    }
}
