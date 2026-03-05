use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    process::Stdio,
};

/// Holds optional stdout and stderr redirection targets.
pub struct Redirections {
    pub stdout_file: Option<File>,
    pub stderr_file: Option<File>,
}

impl Redirections {
    /// Write a line to the stdout redirect file, or to real stdout.
    pub fn write_out(&mut self, output: &str) {
        match self.stdout_file {
            Some(ref mut f) => {
                let _ = writeln!(f, "{}", output);
            }
            None => {
                println!("{}", output);
            }
        }
    }

    /// Write a line to the stderr redirect file, or to real stderr.
    pub fn write_err(&mut self, output: &str) {
        match self.stderr_file {
            Some(ref mut f) => {
                let _ = writeln!(f, "{}", output);
            }
            None => {
                eprintln!("{}", output);
            }
        }
    }

    /// Clone the stdout file handle as a `Stdio`, if redirected.
    pub fn stdout_stdio(&self) -> Option<Stdio> {
        self.stdout_file.as_ref().map(|f| {
            Stdio::from(f.try_clone().expect("failed to clone stdout redirect handle"))
        })
    }

    /// Clone the stderr file handle as a `Stdio`, if redirected.
    pub fn stderr_stdio(&self) -> Option<Stdio> {
        self.stderr_file.as_ref().map(|f| {
            Stdio::from(f.try_clone().expect("failed to clone stderr redirect handle"))
        })
    }
}

/// Parse redirection operators from tokens, returning (command_tokens, Redirections).
///
/// Supported operators: `>`, `1>`, `>>`, `1>>`, `2>`, `2>>`.
pub fn parse(tokens: &[String]) -> Result<(Vec<String>, Redirections), String> {
    let mut cmd_tokens = Vec::new();
    let mut stdout_file: Option<File> = None;
    let mut stderr_file: Option<File> = None;

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            ">" | "1>" => {
                let filename = next_filename(tokens, i)?;
                stdout_file = Some(open_truncate(filename)?);
                i += 2;
            }
            ">>" | "1>>" => {
                let filename = next_filename(tokens, i)?;
                stdout_file = Some(open_append(filename)?);
                i += 2;
            }
            "2>" => {
                let filename = next_filename(tokens, i)?;
                stderr_file = Some(open_truncate(filename)?);
                i += 2;
            }
            "2>>" => {
                let filename = next_filename(tokens, i)?;
                stderr_file = Some(open_append(filename)?);
                i += 2;
            }
            _ => {
                cmd_tokens.push(tokens[i].clone());
                i += 1;
            }
        }
    }

    Ok((cmd_tokens, Redirections { stdout_file, stderr_file }))
}

// ── helpers ──────────────────────────────────────────────────────────

fn next_filename<'a>(tokens: &'a [String], pos: usize) -> Result<&'a str, String> {
    tokens
        .get(pos + 1)
        .map(|s| s.as_str())
        .ok_or_else(|| "syntax error: expected filename after redirection".to_string())
}

fn ensure_parent_dirs(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && !parent.exists()
    {
        fs::create_dir_all(parent)
            .map_err(|e| format!("{}: {}", path.display(), e))?;
    }
    Ok(())
}

fn open_truncate(filename: &str) -> Result<File, String> {
    let path = Path::new(filename);
    ensure_parent_dirs(path)?;
    File::create(path).map_err(|e| format!("{}: {}", filename, e))
}

fn open_append(filename: &str) -> Result<File, String> {
    let path = Path::new(filename);
    ensure_parent_dirs(path)?;
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("{}: {}", filename, e))
}
