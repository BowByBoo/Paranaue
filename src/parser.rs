use std::io;

#[derive(Clone, Copy)]
enum State {
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

/// Tokenize the command language used by the Forge foundation.
///
/// This deliberately handles only words, quotes and escapes. Shell operators
/// are not interpreted until their semantics have been designed explicitly.
pub fn parse_words(input: &str) -> io::Result<Vec<String>> {
    let mut state = State::Unquoted;
    let mut escaped = false;
    let mut current = String::new();
    let mut words = Vec::new();
    let mut started = false;

    for ch in input.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            started = true;
            continue;
        }

        match state {
            State::Unquoted => match ch {
                '\\' => escaped = true,
                '\'' => {
                    state = State::SingleQuoted;
                    started = true;
                }
                '"' => {
                    state = State::DoubleQuoted;
                    started = true;
                }
                ch if ch.is_whitespace() => {
                    if started {
                        words.push(std::mem::take(&mut current));
                        started = false;
                    }
                }
                _ => {
                    current.push(ch);
                    started = true;
                }
            },
            State::SingleQuoted => {
                if ch == '\'' {
                    state = State::Unquoted;
                } else {
                    current.push(ch);
                }
            }
            State::DoubleQuoted => match ch {
                '"' => state = State::Unquoted,
                '\\' => escaped = true,
                _ => current.push(ch),
            },
        }
    }

    if escaped {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unfinished escape at end of command",
        ));
    }

    if !matches!(state, State::Unquoted) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unterminated quoted argument",
        ));
    }

    if started {
        words.push(current);
    }

    Ok(words)
}
