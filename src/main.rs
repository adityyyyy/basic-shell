mod shell;

use rustyline::error::ReadlineError;
use shell::builtins::{self, BUILTINS, cmd_history};
use shell::exec;
use shell::redirect;
use shell::tokenizer::tokenize;

fn main() {
    let mut rl = shell::completions::get_reader();
    loop {
        let readline = rl.readline("$ ");
        let input = match readline {
            Ok(line) => line,
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

            if command == "history" {
                cmd_history(&mut rl, &args, &mut redir);
            } else if BUILTINS.contains(&command.as_str()) {
                if let Some(code) = builtins::run(command, &args, &mut redir) {
                    save_history(&mut rl);
                    std::process::exit(code);
                }
            } else {
                exec::run_external(command, &args, &mut redir);
            }
        } else {
            exec::run_pipeline(&stages);
        }
    }

    save_history(&mut rl);
}

#[allow(unused_variables)]
fn save_history(
    rl: &mut rustyline::Editor<shell::completions::MyHelper, rustyline::history::DefaultHistory>,
) {
    #[cfg(feature = "with-file-history")]
    let _ = rl.save_history("history.txt");
}
