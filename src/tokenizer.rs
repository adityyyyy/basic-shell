/// Tokenize input with support for single quotes, double quotes, and backslash escapes.
pub fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_token = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '\'' => {
                chars.next();
                in_token = true;
                let mut closed = false;
                while let Some(&c) = chars.peek() {
                    if c == '\'' {
                        chars.next();
                        closed = true;
                        break;
                    }
                    current.push(c);
                    chars.next();
                }
                if !closed {
                    return Err("syntax error: unterminated single quote".into());
                }
            }
            '"' => {
                chars.next();
                in_token = true;
                let mut closed = false;
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        closed = true;
                        break;
                    }
                    if c == '\\' {
                        chars.next();
                        if let Some(&next) = chars.peek() {
                            match next {
                                '\\' | '"' | '$' | '`' | '\n' => {
                                    current.push(next);
                                    chars.next();
                                }
                                _ => {
                                    current.push('\\');
                                }
                            }
                        } else {
                            current.push('\\');
                        }
                    } else {
                        current.push(c);
                        chars.next();
                    }
                }
                if !closed {
                    return Err("syntax error: unterminated double quote".into());
                }
            }
            '\\' => {
                chars.next();
                in_token = true;
                if let Some(&next) = chars.peek() {
                    current.push(next);
                    chars.next();
                }
            }
            c if c.is_ascii_whitespace() => {
                chars.next();
                if in_token {
                    tokens.push(std::mem::take(&mut current));
                    in_token = false;
                }
            }
            _ => {
                current.push(ch);
                chars.next();
                in_token = true;
            }
        }
    }

    if in_token || !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}
