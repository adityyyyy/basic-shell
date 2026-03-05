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
