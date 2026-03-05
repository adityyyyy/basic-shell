use std::env;

use crate::shell::redirect::Redirections;

pub fn run(redir: &mut Redirections) {
    match env::current_dir() {
        Ok(path) => redir.write_out(&format!("{}", path.display())),
        Err(err) => redir.write_err(&format!("pwd: {}", err)),
    }
}
