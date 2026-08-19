use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus};

/// Execute a native process in the shell's current working directory.
///
/// Forge delegates program lookup, environment inheritance, and process
/// creation to the operating system. Shell-language features such as pipes,
/// redirection, and expansion are deliberately outside this layer.
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

    #[cfg(unix)]
    #[test]
    fn preserves_successful_exit_status() {
        let status = run("true", &[], Path::new(".")).expect("true must execute");
        assert!(status.success());
        assert_eq!(status.code(), Some(0));
    }

    #[cfg(unix)]
    #[test]
    fn preserves_nonzero_exit_status() {
        let args = vec!["7".to_owned()];
        let status = run("false", &args, Path::new(".")).expect("false must execute");
        assert!(!status.success());
        assert_ne!(status.code(), Some(0));
    }

    #[cfg(windows)]
    #[test]
    fn preserves_successful_exit_status() {
        let args = vec!["/C".to_owned(), "exit".to_owned(), "0".to_owned()];
        let status = run("cmd", &args, Path::new(".")).expect("cmd must execute");
        assert!(status.success());
        assert_eq!(status.code(), Some(0));
    }

    #[cfg(windows)]
    #[test]
    fn preserves_nonzero_exit_status() {
        let args = vec!["/C".to_owned(), "exit".to_owned(), "7".to_owned()];
        let status = run("cmd", &args, Path::new(".")).expect("cmd must execute");
        assert!(!status.success());
        assert_eq!(status.code(), Some(7));
    }

    #[cfg(unix)]
    #[test]
    fn executes_in_the_requested_working_directory() {
        let base = unique_temp_path("forge-process-cwd");
        std::fs::create_dir_all(&base).unwrap();
        let expected = base.canonicalize().unwrap();
        let args = vec![
            "-c".to_owned(),
            "test \"$PWD\" = \"$1\"".to_owned(),
            "forge-cwd-check".to_owned(),
            expected.display().to_string(),
        ];
        let status = run("sh", &args, &base).expect("shell must execute");
        assert!(status.success(), "child process did not observe requested cwd");
        std::fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn rejects_a_missing_working_directory_before_launching() {
        let missing = unique_temp_path("forge-missing-cwd");
        let result = run("forge-test-command-that-must-not-exist-7c4b1d9e", &[], &missing);
        assert!(result.is_err());
    }

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("{prefix}-{}-{}", std::process::id(), unique_nonce()))
    }

    fn unique_nonce() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after the Unix epoch")
            .as_nanos()
    }
}
