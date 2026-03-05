#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use std::path::{Path, PathBuf};

/// Search for an executable by name, checking absolute paths and `$PATH`.
pub fn find_executable(cmd: &str) -> Option<PathBuf> {
    if cmd.contains('/') {
        let path = Path::new(cmd);
        if is_executable(path) {
            return Some(path.to_path_buf());
        }
        return None;
    }

    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let full_path = dir.join(cmd);
        if is_executable(&full_path) {
            return Some(full_path);
        }
    }

    None
}

fn is_executable(path: &Path) -> bool {
    match std::fs::metadata(path) {
        Ok(m) if m.is_file() => {
            #[cfg(unix)]
            {
                m.permissions().mode() & 0o111 != 0
            }
            #[cfg(not(unix))]
            {
                true
            }
        }
        _ => false,
    }
}
