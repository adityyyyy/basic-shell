use crate::builtins::BUILTINS;
use crate::path::list_executables;
use rustyline::{
    CompletionType, Context, Editor, Helper, Highlighter, Hinter, Result, Validator,
    completion::{Completer, Pair, extract_word},
    config::Configurer,
    history::DefaultHistory,
};

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct MyHelper {
    executables: Vec<String>,
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let (word_start, prefix) = extract_word(line, pos, None, |c: char| c == ' ');

        let builtins_iter = BUILTINS.iter().map(|s| *s);
        let execs_iter = self.executables.iter().map(|s| s.as_str());

        let mut seen = std::collections::HashSet::new();
        let matches: Vec<Pair> = builtins_iter
            .chain(execs_iter)
            .filter(|cmd| cmd.starts_with(prefix) && seen.insert(*cmd))
            .map(|cmd| {
                let replacement = format!("{} ", cmd);
                Pair {
                    display: replacement.clone(),
                    replacement,
                }
            })
            .collect();

        Ok((word_start, matches))
    }
}

pub fn get_reader() -> Editor<MyHelper, DefaultHistory> {
    let helper = MyHelper {
        executables: list_executables(),
    };

    let mut rl = Editor::<MyHelper, DefaultHistory>::new().expect("failed to create editor");
    rl.set_helper(Some(helper));
    rl.set_completion_type(CompletionType::Circular);

    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        eprintln!("No previous history");
    }

    rl
}
