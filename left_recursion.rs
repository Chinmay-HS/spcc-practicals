use std::io;

fn is_left_recursive(non_terminal: &str, production: &str) -> bool {
    let production = production.trim();
    // Check if production starts with same non-terminal
    production.starts_with(non_terminal)
}

fn main() {
    println!("Enter number of productions:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    println!("Enter productions in format: A -> Aa | b");
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split("->").collect();
        if parts.len() != 2 {
            println!("Invalid format");
            continue;
        }
        let lhs = parts[0].trim();
        let rhs = parts[1].trim();
        let productions: Vec<&str> = rhs.split('|').collect();
        let mut left_recursive = false;
        for prod in productions {
            if is_left_recursive(lhs, prod) {
                left_recursive = true;
                break;
            }
        }
        if left_recursive {
            println!("{} is Left Recursive", lhs);
        } else {
            println!("{} is NOT Left Recursive", lhs);
        }
    }
}
