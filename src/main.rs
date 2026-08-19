use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut shell = Shell::new();

    if let Err(error) = shell.run() {
        eprintln!("forge: fatal error: {error}");
        std::process::exit(1);
    }
}

struct Shell {
    current_dir: PathBuf,
}

impl Shell {
    fn new() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self { current_dir }
    }

    fn run(&mut self) -> io::Result<()> {
        println!("Forge {VERSION}");
        println!("Type 'help' for help. Type 'exit' to quit.");

        loop {
            print!("forge {}> ", display_path(&self.current_dir));
            io::stdout().flush()?;

            let mut input = String::new();
            let bytes = io::stdin().read_line(&mut input)?;
            if bytes == 0 {
                println!();
                break;
            }

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            if !self.execute(input)? {
                break;
            }
        }

        Ok(())
    }

    fn execute(&mut self, input: &str) -> io::Result<bool> {
        let args = parse_words(input);
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
        let target = target.unwrap_or_else(|| {
            if cfg!(windows) { "." } else { "." }
        });

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

fn print_help() {
    println!(
        "Commands:\n  help       Show this help\n  pwd        Print the current directory\n  cd <path>  Change the current directory\n  version    Show the Forge version\n  exit       Exit Forge\n\nAny other command is executed as a native process."
    );
}

fn parse_words(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect()
}

fn display_path(path: &PathBuf) -> String {
    path.display().to_string()
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
