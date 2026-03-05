/// Tokenize input with support for single quotes, double quotes, and backslash escapes.
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_token = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '\'' => {
                chars.next();
                in_token = true;
                // Read until closing single quote (no escaping inside single quotes)
                while let Some(&c) = chars.peek() {
                    if c == '\'' {
                        chars.next();
                        break;
                    }
                    current.push(c);
                    chars.next();
                }
            }
            '"' => {
                chars.next();
                in_token = true;
                // Inside double quotes: backslash escapes \, ", $, newline
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
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
                                    // Backslash is literal if not followed by special char
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
            }
            '\\' => {
                chars.next();
                in_token = true;
                // Outside quotes: backslash escapes the next character
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

    tokens
}
