#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn find_executable(cmd: &str) -> Option<std::path::PathBuf> {
    if cmd.starts_with("/") {
        let path = std::path::Path::new(cmd);
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

fn is_executable(path: &std::path::Path) -> bool {
    std::fs::metadata(path)
        .ok()
        .filter(|m| m.is_file())
        .map(|m| {
            #[cfg(unix)]
            {
                m.permissions().mode() & 0o111 != 0
            }
        })
        .unwrap_or(false)
}
