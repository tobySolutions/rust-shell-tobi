#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        // Wait for user inputc
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        if input.starts_with("exit") {
            let parts: Vec<&str> = input.split_whitespace().collect();

            if parts.len() > 1 {
                if let Ok(code) = parts[1].parse::<i32>() {
                    process::exit(code);
                }
            }

            process::exit(0)
        }

        if input.starts_with("echo") {
            let parts_of_text: Vec<&str> = input.split_whitespace().collect();

            if parts_of_text.len() > 1 {
                println!("{}", parts_of_text[1..].join(" "));
                continue;
            }
        }

        println!("{}: command not found", input);
    }
}
