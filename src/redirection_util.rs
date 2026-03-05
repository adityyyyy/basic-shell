use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

/// Holds optional stdout and stderr redirection targets.
pub struct Redirections {
    pub stdout_file: Option<File>,
    pub stderr_file: Option<File>,
}

/// Open a file for redirection, creating parent directories as needed.
pub fn open_redirect_file(filename: &str) -> Result<File, String> {
    let path = Path::new(filename);
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && !parent.exists()
    {
        fs::create_dir_all(parent).map_err(|e| format!("{}: {}", filename, e))?;
    }
    File::create(path).map_err(|e| format!("{}: {}", filename, e))
}

/// Scan tokens for `>`, `1>`, and `2>`, extract filenames, and return
/// (command_tokens, Redirections). Supports both stdout and stderr redirection.
pub fn parse_redirections(tokens: &[String]) -> Result<(Vec<String>, Redirections), String> {
    let mut cmd_tokens = Vec::new();
    let mut stdout_file: Option<File> = None;
    let mut stderr_file: Option<File> = None;

    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == ">" || tokens[i] == "1>" {
            let filename = tokens
                .get(i + 1)
                .ok_or_else(|| "syntax error: expected filename after redirection".to_string())?;
            stdout_file = Some(open_redirect_file(filename)?);
            i += 2; // skip operator + filename
        } else if tokens[i] == "2>" {
            let filename = tokens
                .get(i + 1)
                .ok_or_else(|| "syntax error: expected filename after redirection".to_string())?;
            stderr_file = Some(open_redirect_file(filename)?);
            i += 2;
        } else {
            cmd_tokens.push(tokens[i].clone());
            i += 1;
        }
    }

    Ok((
        cmd_tokens,
        Redirections {
            stdout_file,
            stderr_file,
        },
    ))
}

/// Write to the redirect file if present, otherwise to stdout.
pub fn write_output(redir: &mut Redirections, output: &str) {
    match redir.stdout_file {
        Some(ref mut f) => {
            let _ = writeln!(f, "{}", output);
        }
        None => {
            println!("{}", output);
        }
    }
}

/// Write to the stderr redirect file if present, otherwise to stderr.
pub fn write_error(redir: &mut Redirections, output: &str) {
    match redir.stderr_file {
        Some(ref mut f) => {
            let _ = writeln!(f, "{}", output);
        }
        None => {
            eprintln!("{}", output);
        }
    }
}
