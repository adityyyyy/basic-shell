mod builtins;
mod completions;
mod dir;
mod path;
mod redirect;
mod tokenizer;

use builtins::BUILTINS;
use path::find_executable;
use redirect::Redirections;
use rustyline::error::ReadlineError;
use std::process::{Child, Command, Stdio};
use tokenizer::tokenize;

fn main() {
    let mut rl = completions::get_reader();
    loop {
        let readline = rl.readline("$ ");
        let input = match readline {
            Ok(line) => {
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };

        let input = input.trim();
        let tokens = match tokenize(input) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("shell: {}", e);
                continue;
            }
        };
        if tokens.is_empty() {
            continue;
        }

        let stages: Vec<&[String]> = tokens.split(|t| t == "|").collect();

        if stages.len() == 1 {
            // Single command — existing behavior
            let (cmd_tokens, mut redir) = match redirect::parse(&tokens) {
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

            if BUILTINS.contains(&command.as_str()) {
                if let Some(code) = builtins::run(command, &args, &mut redir) {
                    save_history(&mut rl);
                    std::process::exit(code);
                }
            } else {
                run_external(command, &args, &mut redir);
            }
        } else {
            run_pipeline(&stages);
        }
    }

    save_history(&mut rl);
}

#[allow(unused_variables)]
fn save_history(rl: &mut rustyline::Editor<completions::MyHelper, rustyline::history::DefaultHistory>) {
    #[cfg(feature = "with-file-history")]
    let _ = rl.save_history("history.txt");
}

fn run_external(command: &str, args: &[&str], redir: &mut Redirections) {
    if find_executable(command).is_some() {
        let mut cmd = std::process::Command::new(command);
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

fn run_pipeline(stages: &[&[String]]) {
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
                // Mid-pipeline builtin: write output to a pipe
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
                drop(redir.stdout_file.take()); // close write end
                drop(prev_stdout.take());
                prev_stdout = Some(Stdio::from(reader));
            } else {
                // Last-stage builtin: run normally, discard piped stdin
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
                    cmd.stdout(stdio); // redirect takes precedence over pipe
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
