use super::builtins::BUILTINS;
use super::util::path::list_executables;
use rustyline::{
    CompletionType, Context, Editor, Helper, Highlighter, Hinter, Result, Validator,
    completion::{Completer, FilenameCompleter, Pair, extract_word},
    config::Configurer,
    history::DefaultHistory,
};

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct MyHelper {
    executables: Vec<String>,
    filenames: FilenameCompleter,
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let (word_start, prefix) = extract_word(line, pos, None, |c: char| c == ' ');

        let is_first_word = line[..word_start].trim().is_empty();

        if is_first_word {
            let builtins_iter = BUILTINS.iter().copied();
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
        } else {
            let (start, mut pairs) = self.filenames.complete(line, pos, ctx)?;
            for pair in &mut pairs {
                if pair.replacement.ends_with('/') && !pair.display.ends_with('/') {
                    pair.display.push('/');
                }
                if !pair.replacement.ends_with('/') && !pair.replacement.ends_with(' ') {
                    pair.replacement.push(' ');
                }
            }
            Ok((start, pairs))
        }
    }
}

pub fn get_reader() -> Editor<MyHelper, DefaultHistory> {
    let helper = MyHelper {
        executables: list_executables(),
        filenames: FilenameCompleter::new(),
    };

    let mut rl = Editor::<MyHelper, DefaultHistory>::new().expect("failed to create editor");
    rl.set_helper(Some(helper));
    rl.set_completion_type(CompletionType::List);
    rl.set_auto_add_history(true);
    let _ = rl.set_history_ignore_dups(false);

    #[cfg(feature = "with-file-history")]
    if let Ok(histfile) = std::env::var("HISTFILE") {
        let _ = rl.load_history(std::path::Path::new(&histfile));
    }

    rl
}
