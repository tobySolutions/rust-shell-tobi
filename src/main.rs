#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, Command};
use std::env;
use std::path::Path;


fn find_executable(cmd: &str) -> Option<String> {
    let path = env::var("PATH").unwrap_or_default();

    for dir in path.split(":") {
        let full_path = Path::new(dir).join(cmd);

        if full_path.exists() {
            return Some(full_path.to_string_lossy().into_owned());
        }
    }
    None
}

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


        match command {
            "cd" => {
                if parts.len() < 2 {
                    continue;
                }
                
                if parts[1] == "~" {
                    match env::var("HOME") {
                        Ok(home_str) => {
                            if let Err(_) = env::set_current_dir(&home_str) {
                                println!("cd: {}: No such file or directory", parts[1]);
                            }
                        },
                        Err(_) => {
                            println!("cd: HOME not set");
                        }
                    }
                } else {
                    if let Err(_) = env::set_current_dir(parts[1]) {
                        println!("cd: {}: No such file or directory", parts[1]);
                    }
                }
            },
            "pwd" => {
                if let Ok(current_dir) = env::current_dir() {
                    println!("{}", current_dir.display());
                }
            },
            "type" => {
                if parts.len() < 2 {
                    continue;
                }
    
                let cmd_to_check = parts[1];
    
                match cmd_to_check {
                    "echo" | "exit" | "type" | "pwd" => println!("{} is a shell builtin", cmd_to_check),
                    _ => {
                        match find_executable(cmd_to_check) {
                            Some(path) => println!("{} is {}", cmd_to_check, path),
                            None => println!("{}: not found", cmd_to_check),
                        }
                    }
                }
            }, 
            "exit" => {
                if parts.len() > 1 {
                    if let Ok(code) = parts[1].parse::<i32>() {
                        process::exit(code);
                    }
                }
                process::exit(0)
            },
            "echo" => {
                if parts.len() > 1 {
                    println!("{}", parts[1..].join(" "));
                    continue;
                }
            },
            _ => {
                if let Some(cmd_path) = find_executable(command) {
                    let output = Command::new(cmd_path)
                        .args(&parts[1..])
                        .output()
                        .unwrap();
                    
                    io::stdout().write_all(&output.stdout).unwrap();
                    io::stderr().write_all(&output.stderr).unwrap();
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
