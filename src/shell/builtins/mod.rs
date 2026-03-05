mod cd;
mod echo;
mod exit;
mod history;
mod pwd;
mod type_cmd;

use crate::shell::redirect::Redirections;

pub const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd", "history"];

/// Run a builtin command. Returns `Some(exit_code)` if the shell should exit.
pub fn run(command: &str, args: &[&str], redir: &mut Redirections) -> Option<i32> {
    match command {
        "exit" => return exit::run(args, redir),
        "echo" => echo::run(args, redir),
        "type" => type_cmd::run(args, redir),
        "pwd" => pwd::run(redir),
        "cd" => cd::run(args, redir),
        "history" => {} // handled in main.rs (needs editor access)
        _ => unreachable!("not a builtin: {}", command),
    }
    None
}

pub use history::cmd_history;
