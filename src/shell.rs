use std::env;
use std::io;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::{config, history, parser, process, VERSION};

pub struct Shell {
    current_dir: PathBuf,
    config: config::Config,
}

impl Shell {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self { current_dir, config: config::Config::default() }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.config = config::Config::load()?;
        let mut editor = DefaultEditor::new().map_err(|error| io::Error::other(format!("failed to initialize line editor: {error}")))?;
        if let Err(error) = history::load(&mut editor) { eprintln!("forge: warning: {error}"); }
        println!("Forge {VERSION}");
        println!("Type 'help' for help. Type 'exit' to quit.");
        let result = loop {
            let prompt = self.config.prompt(&self.current_dir);
            let input = match editor.readline(&prompt) {
                Ok(input) => input,
                Err(ReadlineError::Interrupted) => { println!(); continue; }
                Err(ReadlineError::Eof) => { println!(); break Ok(()); }
                Err(error) => break Err(io::Error::other(format!("line editor error: {error}"))),
            };
            let input = input.trim();
            if input.is_empty() { continue; }
            if history::enabled() {
                if let Err(error) = editor.add_history_entry(input) { eprintln!("forge: warning: could not record history: {error}"); }
            }
            match self.execute(input) {
                Ok(true) => {}
                Ok(false) => break Ok(()),
                Err(error) => eprintln!("forge: {error}"),
            }
        };
        if let Err(error) = history::save(&mut editor) { eprintln!("forge: warning: {error}"); }
        result
    }

    fn execute(&mut self, input: &str) -> io::Result<bool> {
        let args = parser::parse_words(input)?;
        let Some(command) = args.first().map(String::as_str) else { return Ok(true); };
        match command {
            "exit" | "quit" => Ok(false),
            "help" => { print_help(); Ok(true) }
            "version" | "--version" => { println!("Forge {VERSION}"); Ok(true) }
            "pwd" => { println!("{}", self.current_dir.display()); Ok(true) }
            "cd" => {
                if args.len() > 2 { return Err(io::Error::new(io::ErrorKind::InvalidInput, "cd accepts at most one path argument")); }
                self.change_directory(args.get(1).map(String::as_str))?;
                Ok(true)
            }
            _ => {
                let status = process::run(command, &args[1..], &self.current_dir)?;
                if !status.success() {
                    if let Some(code) = status.code() { eprintln!("forge: '{command}' exited with status {code}"); }
                    else { eprintln!("forge: '{command}' terminated by signal"); }
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
        let path = path.canonicalize().map_err(|error| io::Error::new(error.kind(), format!("cannot change directory to '{}': {error}", path.display())))?;
        if !path.is_dir() { return Err(io::Error::new(io::ErrorKind::NotADirectory, format!("'{}' is not a directory", path.display()))); }
        self.current_dir = path;
        Ok(())
    }
}

impl Default for Shell { fn default() -> Self { Self::new() } }

pub fn print_help() {
    println!("Commands:\n  help       Show this help\n  pwd        Print the current directory\n  cd <path>  Change the current directory\n  version    Show the Forge version\n  exit       Exit Forge\n\nAny other command is executed as a native process.\n\nThe interactive editor provides cursor movement and persistent command history.\nSet FORGE_NO_HISTORY=1 to disable history persistence.\nQuotes and escapes are supported for arguments containing spaces.\n\nConfiguration: config.toml in Forge's per-user configuration directory.\nThe configuration file is declarative and never executes commands.");
}

fn home_directory() -> Option<PathBuf> {
    #[cfg(windows)]
    { env::var_os("USERPROFILE").map(PathBuf::from) }
    #[cfg(not(windows))]
    { env::var_os("HOME").map(PathBuf::from) }
}

#[cfg(test)]
mod tests {
    use super::Shell;
    use std::{env, fs, io};

    fn temp_test_path(name: &str) -> std::path::PathBuf {
        env::temp_dir().join(format!("forge-shell-{name}-{}", std::process::id()))
    }

    #[test]
    fn new_shell_starts_in_current_directory() {
        let shell = Shell::new();
        assert_eq!(shell.current_dir, env::current_dir().unwrap());
    }

    #[test]
    fn change_directory_rejects_more_than_one_argument_at_command_boundary() {
        let mut shell = Shell::new();
        let original = shell.current_dir.clone();
        let result = shell.execute("cd one two");
        assert!(result.is_err());
        assert_eq!(shell.current_dir, original);
    }

    #[test]
    fn change_directory_uses_relative_paths_from_shell_directory() {
        let base = temp_test_path("relative");
        let child = base.join("child");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&child).unwrap();
        let mut shell = Shell::new();
        shell.current_dir = base.clone();
        shell.change_directory(Some("child")).unwrap();
        assert_eq!(shell.current_dir, child.canonicalize().unwrap());
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn change_directory_reports_missing_path() {
        let mut shell = Shell::new();
        let missing = temp_test_path("missing");
        let _ = fs::remove_dir_all(&missing);
        let error = shell.change_directory(Some(missing.to_str().unwrap())).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn change_directory_rejects_a_regular_file() {
        let path = temp_test_path("file");
        let _ = fs::remove_file(&path);
        fs::write(&path, b"not a directory").unwrap();
        let mut shell = Shell::new();
        let error = shell.change_directory(Some(path.to_str().unwrap())).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotADirectory);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn unknown_command_returns_actionable_error() {
        let mut shell = Shell::new();
        let result = shell.execute("forge-command-that-must-not-exist-7c4b1d9e");
        let error = result.expect_err("missing command must return an error");
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert!(error.to_string().contains("command not found"));
    }
}
