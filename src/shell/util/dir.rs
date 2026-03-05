use std::{env, fs, path::PathBuf};

/// Resolve a directory path, expanding `~` to `$HOME`.
pub fn resolve(input: &str) -> Result<PathBuf, String> {
    let path = expand_tilde(input);
    match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(path)
            } else {
                Err(format!("{}: Not a directory", input))
            }
        }
        Err(_) => Err(format!("{}: No such file or directory", input)),
    }
}

fn expand_tilde(input: &str) -> PathBuf {
    input
        .strip_prefix("~")
        .filter(|rest| rest.is_empty() || rest.starts_with("/"))
        .and_then(|rest| {
            env::var_os("HOME")
                .map(|home| PathBuf::from(home).join(rest.strip_prefix("/").unwrap_or("")))
        })
        .unwrap_or_else(|| input.into())
}
