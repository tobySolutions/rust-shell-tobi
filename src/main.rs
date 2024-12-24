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
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];

        if command == "type" {
            if parts.len() < 2 {
                continue;
            }

            let cmd_to_check = parts[1];

            match cmd_to_check {
                "echo" | "exit" | "type" => println!("{} is a shell builtin", cmd_to_check),
                _ => println!("{}: not found", cmd_to_check)
            }

            continue;
        }

        if command == "exit" {
            if parts.len() > 1 {
                if let Ok(code) = parts[1].parse::<i32>() {
                    process::exit(code);
                }
            }
            process::exit(0)
        }   

        if command == "echo" {
            if parts.len() > 1 {
                println!("{}", parts[1..].join(" "));
                continue;
            }
        }


        println!("{}: command not found", input);
    }
}
