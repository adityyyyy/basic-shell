use std::io::{self, Write};

fn main() {
    let mut buf = String::new();

    loop {
        buf.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        let input = buf.trim();
        if input == "exit" {
            break;
        }
        if input.starts_with("echo ") {
            println!("{}", &input[5..]);
        }
        println!("{}: command not found", buf.trim());
    }
}
