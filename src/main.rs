#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut buf = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        println!("{}: command not found", buf.trim());
    }
}
