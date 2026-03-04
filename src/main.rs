use std::io::{self, Write};

fn main() {
    let mut buf = String::new();

    loop {
        buf.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        let input = buf.trim();

        let available_commands = ["exit", "echo", "type"];

        let command = input.split_once(" ").map(|v| v.0).unwrap();

        match command {
            "exit" => {
                break;
            }
            "echo" => {
                println!("{}", &input[5..]);
            }
            "type" => {
                if available_commands.contains(&&input[5..]) {
                    println!("{} is a shell builtin", &input[5..]);
                } else {
                    println!("{}: not found", &input[5..]);
                }
            }
            _ => {
                println!("{command}: command not found");
            }
        };
    }
}
