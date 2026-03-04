use std::io::{self, Write};

fn main() {
    let mut buf = String::new();

    loop {
        buf.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        if buf.trim() == "exit" {
            break;
        }
        println!("{}: command not found", buf.trim());
    }
}
