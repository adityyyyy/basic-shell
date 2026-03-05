use std::process::{Child, Command, Stdio};

use super::builtins::{self, BUILTINS};
use super::redirect::{self, Redirections};
use super::util::path::find_executable;

pub fn run_external(command: &str, args: &[&str], redir: &mut Redirections) {
    if find_executable(command).is_some() {
        let mut cmd = Command::new(command);
        cmd.args(args);

        if let Some(stdio) = redir.stdout_stdio() {
            cmd.stdout(stdio);
        }
        if let Some(stdio) = redir.stderr_stdio() {
            cmd.stderr(stdio);
        }

        match cmd.status() {
            Ok(_) => {}
            Err(err) => redir.write_err(&format!("{}: {}", command, err)),
        }
    } else {
        redir.write_err(&format!("{}: command not found", command));
    }
}

pub fn run_pipeline(stages: &[&[String]]) {
    let mut children: Vec<Child> = Vec::new();
    let mut prev_stdout: Option<Stdio> = None;

    for (i, stage) in stages.iter().enumerate() {
        let is_last = i == stages.len() - 1;

        let (cmd_tokens, mut redir) = match redirect::parse(stage) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("shell: {}", e);
                break;
            }
        };

        if cmd_tokens.is_empty() {
            eprintln!("shell: syntax error near unexpected token `|'");
            break;
        }

        let command = &cmd_tokens[0];
        let args: Vec<&str> = cmd_tokens[1..].iter().map(|s| s.as_str()).collect();

        if BUILTINS.contains(&command.as_str()) {
            if !is_last {
                let (reader, writer) = match std::io::pipe() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("shell: pipe: {}", e);
                        break;
                    }
                };
                let file = std::fs::File::from(std::os::fd::OwnedFd::from(writer));
                redir.stdout_file = Some(file);
                builtins::run(command, &args, &mut redir);
                drop(redir.stdout_file.take());
                drop(prev_stdout.take());
                prev_stdout = Some(Stdio::from(reader));
            } else {
                drop(prev_stdout.take());
                builtins::run(command, &args, &mut redir);
            }
        } else if find_executable(command).is_some() {
            let mut cmd = Command::new(command);
            cmd.args(&args);

            if let Some(stdin) = prev_stdout.take() {
                cmd.stdin(stdin);
            }

            if !is_last {
                if let Some(stdio) = redir.stdout_stdio() {
                    cmd.stdout(stdio);
                } else {
                    cmd.stdout(Stdio::piped());
                }
            } else if let Some(stdio) = redir.stdout_stdio() {
                cmd.stdout(stdio);
            }

            if let Some(stdio) = redir.stderr_stdio() {
                cmd.stderr(stdio);
            }

            match cmd.spawn() {
                Ok(mut child) => {
                    if !is_last {
                        prev_stdout = child.stdout.take().map(Stdio::from);
                    }
                    children.push(child);
                }
                Err(e) => {
                    redir.write_err(&format!("{}: {}", command, e));
                    break;
                }
            }
        } else {
            redir.write_err(&format!("{}: command not found", command));
            drop(prev_stdout.take());
            break;
        }
    }

    for child in &mut children {
        let _ = child.wait();
    }
}
