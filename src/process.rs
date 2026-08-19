use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub fn run(program: &str, args: &[String], current_dir: &Path) -> io::Result<ExitStatus> {
    Command::new(program)
        .args(args)
        .current_dir(current_dir)
        .status()
        .map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("command not found: {program}"),
                )
            } else {
                io::Error::new(
                    error.kind(),
                    format!("failed to execute '{program}': {error}"),
                )
            }
        })
}
