use std::env;
use std::io;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::{history, parser, process, VERSION};

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

        if let Err(error) = history::load(&mut editor) {
            eprintln!("forge: warning: {error}");
        }

        println!("Forge {VERSION}");
        println!("Type 'help' for help. Type 'exit' to quit.");

        let result = loop {
            let prompt = format!("forge {}> ", self.current_dir.display());
            let input = match editor.readline(&prompt) {
                Ok(input) => input,
                Err(ReadlineError::Interrupted) => {
                    println!();
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!();
                    break Ok(());
                }
                Err(error) => break Err(io::Error::other(format!("line editor error: {error}"))),
            };

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            if history::enabled() {
                if let Err(error) = editor.add_history_entry(input) {
                    eprintln!("forge: warning: could not record history: {error}");
                }
            }

            match self.execute(input) {
                Ok(true) => {}
                Ok(false) => break Ok(()),
                Err(error) => eprintln!("forge: {error}"),
            }
        };

        if let Err(error) = history::save(&mut editor) {
            eprintln!("forge: warning: {error}");
        }

        result
    }

    fn execute(&mut self, input: &str) -> io::Result<bool> {
        let args = parser::parse_words(input)?;
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
                let status = process::run(command, &args[1..], &self.current_dir)?;
                if !status.success() {
                    if let Some(code) = status.code() {
                        eprintln!("forge: '{command}' exited with status {code}");
                    } else {
                        eprintln!("forge: '{command}' terminated by signal");
                    }
                }
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
            if path.is_absolute() { path } else { self.current_dir.join(path) }
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
}

impl Default for Shell {
    fn default() -> Self { Self::new() }
}

fn print_help() {
    println!(
        "Commands:\n  help       Show this help\n  pwd        Print the current directory\n  cd <path>  Change the current directory\n  version    Show the Forge version\n  exit       Exit Forge\n\nAny other command is executed as a native process.\n\nThe interactive editor provides cursor movement and persistent command history.\nSet FORGE_NO_HISTORY=1 to disable history persistence.\nQuotes and escapes are supported for arguments containing spaces."
    );
}

fn home_directory() -> Option<PathBuf> {
    #[cfg(windows)]
    { env::var_os("USERPROFILE").map(PathBuf::from) }

    #[cfg(not(windows))]
    { env::var_os("HOME").map(PathBuf::from) }
}
