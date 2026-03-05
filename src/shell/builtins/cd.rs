use std::env;

use crate::shell::redirect::Redirections;
use crate::shell::util::dir;

pub fn run(args: &[&str], redir: &mut Redirections) {
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
