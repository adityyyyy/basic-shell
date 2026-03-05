use crate::shell::redirect::Redirections;

pub fn run(args: &[&str], redir: &mut Redirections) -> Option<i32> {
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
