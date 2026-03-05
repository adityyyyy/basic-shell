use crate::shell::redirect::Redirections;

pub fn run(args: &[&str], redir: &mut Redirections) {
    redir.write_out(&args.join(" "));
}
