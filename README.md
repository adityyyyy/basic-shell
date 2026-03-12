# codecrafters-shell

A Unix-like shell built from scratch in Rust, created as a [CodeCrafters](https://codecrafters.io/) challenge solution. It supports builtin and external commands, pipelines, I/O redirections, tab completion, and persistent history.

## Features

- **Builtin commands**: `echo`, `exit`, `cd`, `pwd`, `type`, `history`
- **External commands**: searches `$PATH` or accepts absolute paths
- **Pipelines**: chain commands with `|` (e.g. `ls | grep foo | wc -l`)
- **I/O redirections**: `>`, `>>`, `1>`, `1>>`, `2>`, `2>>`
- **Tab completion**: builtins, `$PATH` executables, and filenames/directories
- **Command history**: in-memory + optional persistent file via `HISTFILE`
- **Quoting & escaping**: single quotes, double quotes (with `\\`, `\"`, `\$`, `` \` ``, `\n`), and backslash escapes

## Builtin Commands

| Command | Usage | Description |
|---------|-------|-------------|
| `echo`  | `echo [args...]` | Print arguments to stdout |
| `exit`  | `exit [code]` | Exit the shell (default code: 0) |
| `cd`    | `cd [dir]` | Change directory; supports `~` expansion, defaults to `$HOME` |
| `pwd`   | `pwd` | Print working directory |
| `type`  | `type command` | Show whether a command is a builtin or its executable path |
| `history` | `history [N \| -c \| -a file \| -w file \| -r file]` | Manage command history |

### History Options

| Flag | Description |
|------|-------------|
| *(none)* | List all history with line numbers |
| `N` | Show last N entries |
| `-c` | Clear history |
| `-a file` | Append new (unflushed) entries to file |
| `-w file` | Write all history to file |
| `-r file` | Read and load history from file |

## Building & Running

```sh
cargo build --release
./target/release/codecrafters-shell

# or for development
cargo run
```

Set `HISTFILE` to persist history across sessions:

```sh
export HISTFILE=~/.shell_history
cargo run
```

## Project Structure

```
src/
├── main.rs                    # REPL loop, history saving
└── shell/
    ├── mod.rs
    ├── builtins/
    │   ├── mod.rs             # Builtin registry & dispatch
    │   ├── echo.rs
    │   ├── exit.rs
    │   ├── cd.rs
    │   ├── pwd.rs
    │   ├── type_cmd.rs
    │   └── history.rs
    ├── exec.rs                # External command & pipeline execution
    ├── redirect.rs            # I/O redirection parsing
    ├── tokenizer.rs           # Tokenizer with quote/escape support
    ├── completions.rs         # Tab completion (commands + filenames)
    └── util/
        ├── mod.rs
        ├── path.rs            # $PATH searching & executable listing
        └── dir.rs             # Directory resolution & ~ expansion
```

## Dependencies

- **Rust** 2024 edition (MSRV 1.91)
- [rustyline](https://crates.io/crates/rustyline) 17.0.2 — line editing, history, and tab completion

## Cargo Features

| Feature | Default | Description |
|---------|---------|-------------|
| `with-file-history` | yes | Enable persistent history via `HISTFILE` |

## Roadmap

### Phase 1 — Core Shell Fundamentals

- [ ] **Variable expansion** — `$VAR`, `${VAR}`, `$?` (last exit code), `$$` (PID)
- [ ] **Stdin redirection** — `<` and heredocs (`<<`, `<<<`)
- [ ] **Command chaining** — `&&`, `||`, `;` operators
- [ ] **Globbing** — `*`, `?`, `[...]` pattern expansion
- [ ] **Fix `history -d`** — currently a stub that doesn't delete entries

### Phase 2 — Job Control & Signals

- [ ] **Background execution** — `&` operator, `jobs`, `fg`, `bg` builtins
- [ ] **Signal handling** — proper SIGINT/SIGTSTP/SIGCHLD forwarding to child processes
- [ ] **Process group management** — `setpgid`/`tcsetpgrp` for correct terminal ownership

### Phase 3 — Scripting Support

- [ ] **Control flow** — `if`/`else`/`fi`, `while`/`do`/`done`, `for`/`in`/`do`/`done`
- [ ] **Functions** — user-defined shell functions
- [ ] **Script execution** — run `.sh` files from arguments (`./myshell script.sh`)
- [ ] **`source` / `.` builtin** — execute scripts in the current shell context
- [ ] **Exit codes** — consistent `$?` propagation across pipelines and builtins

### Phase 4 — Environment & Configuration

- [ ] **`export` / `unset` / `set` builtins** — environment variable management
- [ ] **Startup files** — `~/.myshellrc` loaded at startup
- [ ] **Aliases** — `alias`/`unalias` builtins
- [ ] **Prompt customization** — configurable `PS1` with escape sequences (username, hostname, cwd, git branch)

### Phase 5 — Improved UX

- [ ] **Dynamic completions** — rescan `$PATH` on demand instead of caching once at startup
- [ ] **Syntax highlighting** — colorize input as the user types (via `rustyline`'s `Highlighter` trait)
- [ ] **Smarter history** — deduplication, substring search (`Ctrl-R`), per-directory history
- [ ] **`help` builtin** — usage info for all builtins
- [ ] **Pipeline stdin for builtins** — allow builtins like `echo` to read piped input

### Phase 6 — Advanced Features

- [ ] **Command substitution** — `` `cmd` `` and `$(cmd)`
- [ ] **Process substitution** — `<(cmd)`, `>(cmd)`
- [ ] **Arithmetic expansion** — `$((expr))`
- [ ] **Subshells** — `(cmd1; cmd2)` grouping
- [ ] **`trap` builtin** — custom signal handlers
- [ ] **`test` / `[` builtin** — conditional expressions

### Phase 7 — Polish & Distribution

- [ ] **Comprehensive error messages** — contextual suggestions on typos (did-you-mean)
- [ ] **POSIX compliance testing** — run against a conformance suite
- [ ] **Cross-platform support** — Windows via `windows-sys` (or scope to Unix-only explicitly)
- [ ] **CI/CD & packaging** — GitHub Actions, `cargo install`, Homebrew formula
- [ ] **Documentation** — man page, `--help` flag, contributor guide
