mod executable_file_absolute_path_resolver;
mod tokenizer;
mod utils;

use executable_file_absolute_path_resolver::find_executable;
use std::{
    env,
    io::{self, Write},
};
use tokenizer::tokenize;
use utils::resolve_directory;

const BUILTIN_COMMANDS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

fn main() {
    let mut buf = String::new();

    loop {
        buf.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {}
            Err(err) => {
                eprintln!("shell: failed to read input: {}", err);
                break;
            }
        }
        let input = buf.trim();
        if input.is_empty() {
            continue;
        }

        let tokens = tokenize(input);
        if tokens.is_empty() {
            continue;
        }

        let command = &tokens[0];
        let args: Vec<&str> = tokens[1..].iter().map(|s| s.as_str()).collect();

        match command.as_str() {
            "exit" => break,
            "echo" => {
                println!("{}", args.join(" "));
            }
            "type" => {
                if let Some(cmd) = args.first() {
                    if BUILTIN_COMMANDS.contains(cmd) {
                        println!("{cmd} is a shell builtin");
                    } else if let Some(path) = find_executable(cmd) {
                        println!("{} is {}", cmd, path.display());
                    } else {
                        println!("{}: not found", cmd);
                    }
                } else {
                    eprintln!("type: missing argument");
                }
            }
            "pwd" => match env::current_dir() {
                Ok(path) => {
                    println!("{}", path.display());
                }
                Err(err) => {
                    eprintln!("pwd: {}", err);
                }
            },
            "cd" => {
                let target = if let Some(dir) = args.first() {
                    dir.to_string()
                } else {
                    // cd with no args goes to $HOME
                    match env::var("HOME") {
                        Ok(home) => home,
                        Err(_) => {
                            eprintln!("cd: HOME not set");
                            continue;
                        }
                    }
                };
                match resolve_directory(&target) {
                    Ok(path) => {
                        if let Err(err) = env::set_current_dir(&path) {
                            eprintln!("cd: {}: {}", target, err);
                        }
                    }
                    Err(err) => {
                        eprintln!("cd: {}", err);
                    }
                }
            }
            _ => {
                if find_executable(command).is_some() {
                    match std::process::Command::new(command.as_str())
                        .args(&args)
                        .status()
                    {
                        Ok(_status) => {}
                        Err(err) => {
                            eprintln!("{}: {}", command, err);
                        }
                    }
                } else {
                    println!("{}: command not found", command);
                }
            }
        };
    }
}
