use rustyline::{
    CompletionType, Context, Editor, Helper, Highlighter, Hinter, Result, Validator,
    completion::{Completer, Pair, extract_word},
    config::Configurer,
    history::DefaultHistory,
};

const COMPLETION_COMMANDS: &[&str] = &["echo", "exit"];

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct MyHelper;

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let (word_start, prefix) = extract_word(line, pos, None, |c: char| c == ' ');

        let matches: Vec<Pair> = COMPLETION_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|cmd| {
                let mut com = cmd.to_string();
                com.push(' ');
                Pair {
                    display: com.clone(),
                    replacement: com.clone(),
                }
            })
            .collect();

        Ok((word_start, matches))
    }
}

pub fn get_reader() -> Editor<MyHelper, DefaultHistory> {
    let helper = MyHelper {};

    let mut rl = Editor::<MyHelper, DefaultHistory>::new().expect("failed to create editor");
    rl.set_helper(Some(helper));
    rl.set_completion_type(CompletionType::Circular);

    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        eprintln!("No previous history");
    }

    rl
}
