use crate::shell::builtins::BUILTINS;
use crate::shell::redirect::Redirections;
use crate::shell::util::path::find_executable;

pub fn run(args: &[&str], redir: &mut Redirections) {
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
