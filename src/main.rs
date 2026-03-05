mod executable_file_absolute_path_resolver;
mod tokenizer;
mod utils;

use executable_file_absolute_path_resolver::find_executable;
use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};
use tokenizer::tokenize;
use utils::resolve_directory;

const BUILTIN_COMMANDS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

/// Represents a parsed redirection target for stdout.
struct Redirection {
    file: File,
}

/// Scan tokens for `>` or `1>`, extract the filename, and return
/// (command_tokens, Option<Redirection>). Creates parent dirs as needed.
fn parse_redirection(tokens: &[String]) -> Result<(Vec<String>, Option<Redirection>), String> {
    // Find the first `>` or `1>` token
    let redir_pos = tokens.iter().position(|t| t == ">" || t == "1>");

    match redir_pos {
        Some(pos) => {
            let filename = tokens
                .get(pos + 1)
                .ok_or_else(|| "syntax error: expected filename after redirection".to_string())?;

            // Create parent directories if they don't exist
            let path = Path::new(filename);
            if let Some(parent) = path.parent()
                && !parent.as_os_str().is_empty()
                && !parent.exists()
            {
                fs::create_dir_all(parent).map_err(|e| format!("{}: {}", filename, e))?;
            }

            let file = File::create(path).map_err(|e| format!("{}: {}", filename, e))?;

            // Everything before the redirection operator is the command + args
            let cmd_tokens = tokens[..pos].to_vec();
            Ok((cmd_tokens, Some(Redirection { file })))
        }
        None => Ok((tokens.to_vec(), None)),
    }
}

/// Write a line to the redirect file if present, otherwise to stdout.
fn write_output(redir: &mut Option<Redirection>, output: &str) {
    match redir {
        Some(r) => {
            let _ = writeln!(r.file, "{}", output);
        }
        None => {
            println!("{}", output);
        }
    }
}

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

        // Parse redirection before dispatching the command
        let (cmd_tokens, mut redir) = match parse_redirection(&tokens) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("shell: {}", err);
                continue;
            }
        };

        if cmd_tokens.is_empty() {
            continue;
        }

        let command = &cmd_tokens[0];
        let args: Vec<&str> = cmd_tokens[1..].iter().map(|s| s.as_str()).collect();

        match command.as_str() {
            "exit" => break,
            "echo" => {
                write_output(&mut redir, &args.join(" "));
            }
            "type" => {
                if let Some(cmd) = args.first() {
                    if BUILTIN_COMMANDS.contains(cmd) {
                        write_output(&mut redir, &format!("{cmd} is a shell builtin"));
                    } else if let Some(path) = find_executable(cmd) {
                        write_output(&mut redir, &format!("{} is {}", cmd, path.display()));
                    } else {
                        write_output(&mut redir, &format!("{}: not found", cmd));
                    }
                } else {
                    eprintln!("type: missing argument");
                }
            }
            "pwd" => match env::current_dir() {
                Ok(path) => {
                    write_output(&mut redir, &format!("{}", path.display()));
                }
                Err(err) => {
                    eprintln!("pwd: {}", err);
                }
            },
            "cd" => {
                let target = if let Some(dir) = args.first() {
                    dir.to_string()
                } else {
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
                    let mut cmd = std::process::Command::new(command.as_str());
                    cmd.args(&args);

                    // Redirect stdout to file if `>` / `1>` was used
                    if let Some(ref redir) = redir {
                        let stdout_file = redir
                            .file
                            .try_clone()
                            .expect("failed to clone redirect file handle");
                        cmd.stdout(std::process::Stdio::from(stdout_file));
                    }

                    match cmd.status() {
                        Ok(_status) => {}
                        Err(err) => {
                            eprintln!("{}: {}", command, err);
                        }
                    }
                } else {
                    write_output(&mut redir, &format!("{}: command not found", command));
                }
            }
        };
    }
}
