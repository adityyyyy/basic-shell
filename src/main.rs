use std::io::{self, Write};

fn main() {
    let mut buf = String::new();

    loop {
        buf.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut buf).unwrap() == 0 {
            continue;
        };
        let input = buf.trim();
        if input.is_empty() {
            continue;
        }

        let available_commands = ["exit", "echo", "type"];

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => break,
            "echo" => {
                println!("{}", args.join(" "));
            }
            "type" => {
                if let Some(cmd) = args.first() {
                    if available_commands.contains(cmd) {
                        println!("{cmd} is a shell builtin");
                    } else {
                        println!("{cmd}: not found");
                    }
                }
            }
            _ => {
                println!("{}: command not found", command);
            }
        };
    }
}
