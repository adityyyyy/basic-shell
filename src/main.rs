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
use tokenizer::tokenize;

fn main() {
    let mut rl = completions::get_reader();
    loop {
        let readline = rl.readline("$ ");
        let input = match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
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
        let tokens = tokenize(input);
        if tokens.is_empty() {
            continue;
        }

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
            if builtins::run(command, &args, &mut redir) {
                break;
            }
        } else {
            run_external(command, &args, &mut redir);
        }
    }

    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
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
        redir.write_out(&format!("{}: command not found", command));
    }
}
