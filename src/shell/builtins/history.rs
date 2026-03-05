use std::path::Path;
use std::{fs::OpenOptions, io::Write};

use crate::shell::completions::MyHelper;
use crate::shell::redirect::Redirections;
use rustyline::history::{DefaultHistory, History};

pub type Rl = rustyline::Editor<MyHelper, DefaultHistory>;

pub fn cmd_history(rl: &mut Rl, args: &[&str], redir: &mut Redirections, last_flushed: &mut usize) {
    match args.first().copied() {
        Some("-c") => {
            if rl.clear_history().is_err() {
                redir.write_err("history: failed to clear history");
            }
        }
        Some("-d") => {
            if let Some(offset) = args.get(1) {
                match offset.parse::<usize>() {
                    Ok(n) if n >= 1 && n <= rl.history().len() => {
                        redir.write_err("history: -d requires mutable access (not yet supported)");
                    }
                    _ => redir.write_err(&format!("history: {}: invalid option", offset)),
                }
            } else {
                redir.write_err("history: -d: option requires an argument");
            }
        }
        Some("-a") => {
            if let Some(filename) = args.get(1) {
                let entries: Vec<_> = rl.history().iter().collect();
                if *last_flushed < entries.len() {
                    match OpenOptions::new().create(true).append(true).open(filename) {
                        Ok(mut f) => {
                            for entry in &entries[*last_flushed..] {
                                let _ = writeln!(f, "{}", entry);
                            }
                            *last_flushed = entries.len();
                        }
                        Err(e) => redir.write_err(&format!("history: {}: {}", filename, e)),
                    }
                }
            } else {
                redir.write_err("history: -a: option requires an argument");
            }
        }
        Some("-r") => {
            if let Some(filename) = args.get(1) {
                if rl.load_history(Path::new(filename)).is_err() {
                    redir.write_err(&format!("history: {}: cannot read", filename));
                }
            } else {
                redir.write_err("history: -r: option requires an argument");
            }
        }
        Some("-w") => {
            if let Some(filename) = args.get(1) {
                match std::fs::File::create(filename) {
                    Ok(mut f) => {
                        for entry in rl.history().iter() {
                            let _ = writeln!(f, "{}", entry);
                        }
                    }
                    Err(e) => redir.write_err(&format!("history: {}: {}", filename, e)),
                }
            } else {
                redir.write_err("history: -w: option requires an argument");
            }
        }
        Some(n_str) => match n_str.parse::<usize>() {
            Ok(n) => {
                let entries: Vec<_> = rl.history().iter().collect();
                let start = entries.len().saturating_sub(n);
                for (i, entry) in entries[start..].iter().enumerate() {
                    redir.write_out(&format!("{:>4}  {}", start + i + 1, entry));
                }
            }
            Err(_) => redir.write_err(&format!("history: {}: numeric argument required", n_str)),
        },
        None => {
            for (i, entry) in rl.history().iter().enumerate() {
                redir.write_out(&format!("{:>4}  {}", i + 1, entry));
            }
        }
    }
}
