mod config;
mod history;
mod parser;
mod paths;
mod process;
mod shell;

pub use parser::parse_words;
pub use shell::{print_help, Shell};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::{parse_words, process};
    use std::path::Path;

    #[test]
    fn parses_simple_arguments() {
        assert_eq!(parse_words("echo hello world").unwrap(), ["echo", "hello", "world"]);
    }

    #[test]
    fn preserves_spaces_inside_quotes() {
        assert_eq!(parse_words("echo \"hello world\"").unwrap(), ["echo", "hello world"]);
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
    fn supports_escaped_quote_in_double_quotes() {
        assert_eq!(parse_words("echo \"hello \\\"world\\\"\"").unwrap(), ["echo", "hello \"world\""]);
    }

    #[test]
    fn supports_adjacent_quoted_and_unquoted_text() {
        assert_eq!(parse_words("echo pre\"mid\"post").unwrap(), ["echo", "premidpost"]);
    }

    #[test]
    fn keeps_empty_quoted_argument() {
        assert_eq!(parse_words("echo \"\"").unwrap(), ["echo", ""]);
    }

    #[test]
    fn ignores_whitespace_between_arguments() {
        assert_eq!(parse_words("  echo   hello\tworld  ").unwrap(), ["echo", "hello", "world"]);
    }

    #[test]
    fn empty_input_produces_no_arguments() {
        assert!(parse_words("   \t\n").unwrap().is_empty());
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
        assert_eq!(parse_words("echo olá mundo 🌎").unwrap(), ["echo", "olá", "mundo", "🌎"]);
    }

    #[cfg(unix)]
    #[test]
    fn parsed_arguments_reach_native_process_without_reinterpretation() {
        let args = parse_words("printf '%s' 'hello world'").unwrap();
        let status = process::run(&args[0], &args[1..], Path::new("."))
            .expect("printf should execute on Unix");
        assert!(status.success());
    }

    #[cfg(unix)]
    #[test]
    fn native_process_success_is_observable() {
        let status = process::run("true", &[], Path::new("."))
            .expect("the platform-provided true command should execute");
        assert!(status.success());
    }

    #[cfg(unix)]
    #[test]
    fn native_process_failure_preserves_exit_code() {
        let status = process::run("false", &[], Path::new("."))
            .expect("the platform-provided false command should execute");
        assert!(!status.success());
        assert_eq!(status.code(), Some(1));
    }

    #[cfg(windows)]
    #[test]
    fn native_process_success_is_observable() {
        let args = vec!["/C".to_owned(), "exit 0".to_owned()];
        let status = process::run("cmd", &args, Path::new("."))
            .expect("cmd should be available on Windows");
        assert!(status.success());
    }

    #[cfg(windows)]
    #[test]
    fn native_process_failure_preserves_exit_code() {
        let args = vec!["/C".to_owned(), "exit 7".to_owned()];
        let status = process::run("cmd", &args, Path::new("."))
            .expect("cmd should be available on Windows");
        assert!(!status.success());
        assert_eq!(status.code(), Some(7));
    }
}
