use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn find_executable(cmd: &str) -> Option<PathBuf> {
    if cmd.starts_with("/") {
        let path = Path::new(cmd);
        if is_executable(path) {
            return Some(path.to_path_buf());
        }
        return None;
    }

    let paths = env::var_os("PATH")?;
    for dir in env::split_paths(&paths) {
        let full_path = dir.join(cmd);
        if is_executable(&full_path) {
            return Some(full_path);
        }
    }

    None
}

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_file() {
            #[cfg(unix)]
            {
                let mode = metadata.permissions().mode();
                return mode & 0o111 != 0;
            }
        }
    }

    false
}

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
                        continue;
                    }
                    if let Some(path) = find_executable(cmd) {
                        println!("{} is {}", cmd, path.display());
                    } else {
                        println!("{}: not found", cmd);
                    }
                }
            }
            _ => {
                println!("{}: command not found", command);
            }
        };
    }
}
