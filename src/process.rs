use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus};

/// Execute a native process in the shell's current working directory.
///
/// Forge intentionally delegates program lookup and process creation to the
/// operating system. Shell-language features such as pipes and redirection
/// are not part of this layer.
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

#[cfg(test)]
mod tests {
    use super::run;
    use std::path::Path;

    #[test]
    fn reports_missing_program_without_panicking() {
        let result = run(
            "forge-test-command-that-must-not-exist-7c4b1d9e",
            &[],
            Path::new("."),
        );

        let error = result.expect_err("a deliberately missing program must fail");
        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        assert!(error.to_string().contains("command not found"));
    }

    #[cfg(unix)]
    #[test]
    fn rejects_invalid_program_name() {
        let result = run("\0", &[], Path::new("."));
        assert!(result.is_err());
    }
}
