mod executable_file_absolute_path_resolver;
mod redirection_util;
mod tokenizer;
mod utils;

use executable_file_absolute_path_resolver::find_executable;
use redirection_util::{parse_redirections, write_error, write_output};
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

        // Parse redirections (>, 1>, 2>) before dispatching the command
        let (cmd_tokens, mut redir) = match parse_redirections(&tokens) {
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
                    write_error(&mut redir, "type: missing argument");
                }
            }
            "pwd" => match env::current_dir() {
                Ok(path) => {
                    write_output(&mut redir, &format!("{}", path.display()));
                }
                Err(err) => {
                    write_error(&mut redir, &format!("pwd: {}", err));
                }
            },
            "cd" => {
                let target = if let Some(dir) = args.first() {
                    dir.to_string()
                } else {
                    match env::var("HOME") {
                        Ok(home) => home,
                        Err(_) => {
                            write_error(&mut redir, "cd: HOME not set");
                            continue;
                        }
                    }
                };
                match resolve_directory(&target) {
                    Ok(path) => {
                        if let Err(err) = env::set_current_dir(&path) {
                            write_error(&mut redir, &format!("cd: {}: {}", target, err));
                        }
                    }
                    Err(err) => {
                        write_error(&mut redir, &format!("cd: {}", err));
                    }
                }
            }
            _ => {
                if find_executable(command).is_some() {
                    let mut cmd = std::process::Command::new(command.as_str());
                    cmd.args(&args);

                    // Redirect stdout to file if `>` / `1>` was used
                    if let Some(ref f) = redir.stdout_file {
                        let cloned = f
                            .try_clone()
                            .expect("failed to clone stdout redirect file handle");
                        cmd.stdout(std::process::Stdio::from(cloned));
                    }

                    // Redirect stderr to file if `2>` was used
                    if let Some(ref f) = redir.stderr_file {
                        let cloned = f
                            .try_clone()
                            .expect("failed to clone stderr redirect file handle");
                        cmd.stderr(std::process::Stdio::from(cloned));
                    }

                    match cmd.status() {
                        Ok(_status) => {}
                        Err(err) => {
                            write_error(&mut redir, &format!("{}: {}", command, err));
                        }
                    }
                } else {
                    write_output(&mut redir, &format!("{}: command not found", command));
                }
            }
        };
    }
}
