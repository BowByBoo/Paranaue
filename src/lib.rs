use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Shell {
    current_dir: PathBuf,
}

impl Shell {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self { current_dir }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut editor = DefaultEditor::new()
            .map_err(|error| io::Error::other(format!("failed to initialize line editor: {error}")))?;

        println!("Forge {VERSION}");
        println!("Type 'help' for help. Type 'exit' to quit.");

        loop {
            let prompt = format!("forge {}> ", self.current_dir.display());
            let input = match editor.readline(&prompt) {
                Ok(input) => input,
                Err(ReadlineError::Interrupted) => {
                    println!();
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!();
                    break;
                }
                Err(error) => {
                    return Err(io::Error::other(format!("line editor error: {error}")));
                }
            };

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            if let Err(error) = editor.add_history_entry(input) {
                eprintln!("forge: warning: could not record history: {error}");
            }

            match self.execute(input) {
                Ok(true) => {}
                Ok(false) => break,
                Err(error) => eprintln!("forge: {error}"),
            }
        }

        Ok(())
    }

    fn execute(&mut self, input: &str) -> io::Result<bool> {
        let args = parse_words(input)?;
        let Some(command) = args.first().map(String::as_str) else {
            return Ok(true);
        };

        match command {
            "exit" | "quit" => Ok(false),
            "help" => {
                print_help();
                Ok(true)
            }
            "version" | "--version" => {
                println!("Forge {VERSION}");
                Ok(true)
            }
            "pwd" => {
                println!("{}", self.current_dir.display());
                Ok(true)
            }
            "cd" => {
                if args.len() > 2 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "cd accepts at most one path argument",
                    ));
                }
                self.change_directory(args.get(1).map(String::as_str))?;
                Ok(true)
            }
            _ => {
                self.run_external(&args)?;
                Ok(true)
            }
        }
    }

    fn change_directory(&mut self, target: Option<&str>) -> io::Result<()> {
        let target = target.unwrap_or("~");
        let path = if target == "~" {
            home_directory().unwrap_or_else(|| self.current_dir.clone())
        } else {
            let path = PathBuf::from(target);
            if path.is_absolute() {
                path
            } else {
                self.current_dir.join(path)
            }
        };

        let path = path.canonicalize().map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("cannot change directory to '{}': {error}", path.display()),
            )
        })?;

        if !path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!("'{}' is not a directory", path.display()),
            ));
        }

        self.current_dir = path;
        Ok(())
    }

    fn run_external(&self, args: &[String]) -> io::Result<()> {
        let command = &args[0];
        let status = Command::new(command)
            .args(&args[1..])
            .current_dir(&self.current_dir)
            .status();

        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(status) => {
                if let Some(code) = status.code() {
                    eprintln!("forge: '{command}' exited with status {code}");
                } else {
                    eprintln!("forge: '{command}' terminated by signal");
                }
                Ok(())
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                eprintln!("forge: command not found: {command}");
                Ok(())
            }
            Err(error) => Err(io::Error::new(
                error.kind(),
                format!("failed to execute '{command}': {error}"),
            )),
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

fn print_help() {
    println!(
        "Commands:\n  help       Show this help\n  pwd        Print the current directory\n  cd <path>  Change the current directory\n  version    Show the Forge version\n  exit       Exit Forge\n\nAny other command is executed as a native process.\n\nThe interactive editor provides cursor movement and command history.\nQuotes and escapes are supported for arguments containing spaces."
    );
}

pub fn parse_words(input: &str) -> io::Result<Vec<String>> {
    #[derive(Clone, Copy)]
    enum State {
        Unquoted,
        SingleQuoted,
        DoubleQuoted,
    }

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

fn home_directory() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        env::var_os("USERPROFILE").map(PathBuf::from)
    }

    #[cfg(not(windows))]
    {
        env::var_os("HOME").map(PathBuf::from)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_words;

    #[test]
    fn parses_simple_arguments() {
        assert_eq!(parse_words("echo hello world").unwrap(), ["echo", "hello", "world"]);
    }

    #[test]
    fn preserves_spaces_inside_quotes() {
        assert_eq!(
            parse_words("echo \"hello world\"").unwrap(),
            ["echo", "hello world"]
        );
    }

    #[test]
    fn supports_single_quotes() {
        assert_eq!(parse_words("echo 'hello world'").unwrap(), ["echo", "hello world"]);
    }

    #[test]
    fn supports_escaped_spaces() {
        assert_eq!(parse_words("echo hello\\ world").unwrap(), ["echo", "hello world"]);
    }

    #[test]
    fn rejects_unterminated_quotes() {
        assert!(parse_words("echo \"hello").is_err());
    }

    #[test]
    fn rejects_trailing_escape() {
        assert!(parse_words("echo hello\\").is_err());
    }

    #[test]
    fn handles_unicode() {
        assert_eq!(
            parse_words("echo olá mundo 🌎").unwrap(),
            ["echo", "olá", "mundo", "🌎"]
        );
    }
}
