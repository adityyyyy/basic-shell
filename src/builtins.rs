use std::env;

use crate::dir;
use crate::path::find_executable;
use crate::redirect::Redirections;

pub const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd", "history"];

/// Run a builtin command. Returns `Some(exit_code)` if the shell should exit.
pub fn run(command: &str, args: &[&str], redir: &mut Redirections) -> Option<i32> {
    match command {
        "exit" => return cmd_exit(args, redir),
        "echo" => cmd_echo(args, redir),
        "type" => cmd_type(args, redir),
        "pwd" => cmd_pwd(redir),
        "cd" => cmd_cd(args, redir),
        "history" => {} // handled in main.rs
        _ => unreachable!("not a builtin: {}", command),
    }
    None
}

fn cmd_exit(args: &[&str], redir: &mut Redirections) -> Option<i32> {
    match args.first() {
        Some(s) => match s.parse::<i32>() {
            Ok(code) => Some(code),
            Err(_) => {
                redir.write_err(&format!("exit: {}: numeric argument required", s));
                None
            }
        },
        None => Some(0),
    }
}

fn cmd_echo(args: &[&str], redir: &mut Redirections) {
    redir.write_out(&args.join(" "));
}

fn cmd_type(args: &[&str], redir: &mut Redirections) {
    if let Some(cmd) = args.first() {
        if BUILTINS.contains(cmd) {
            redir.write_out(&format!("{cmd} is a shell builtin"));
        } else if let Some(path) = find_executable(cmd) {
            redir.write_out(&format!("{} is {}", cmd, path.display()));
        } else {
            redir.write_out(&format!("{}: not found", cmd));
        }
    } else {
        redir.write_err("type: missing argument");
    }
}

fn cmd_pwd(redir: &mut Redirections) {
    match env::current_dir() {
        Ok(path) => redir.write_out(&format!("{}", path.display())),
        Err(err) => redir.write_err(&format!("pwd: {}", err)),
    }
}

fn cmd_cd(args: &[&str], redir: &mut Redirections) {
    let target = if let Some(d) = args.first() {
        d.to_string()
    } else {
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                redir.write_err("cd: HOME not set");
                return;
            }
        }
    };
    match dir::resolve(&target) {
        Ok(path) => {
            if let Err(err) = env::set_current_dir(&path) {
                redir.write_err(&format!("cd: {}: {}", target, err));
            }
        }
        Err(err) => redir.write_err(&format!("cd: {}", err)),
    }
}
