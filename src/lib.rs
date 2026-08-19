mod history;
mod parser;
mod process;
mod shell;

pub use parser::parse_words;
pub use shell::Shell;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::parse_words;

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

    #[test]
    fn keeps_empty_quoted_argument() {
        assert_eq!(parse_words("echo \"\"").unwrap(), ["echo", ""]);
    }

    #[test]
    fn supports_adjacent_quoted_and_unquoted_text() {
        assert_eq!(parse_words("echo pre\"mid\"post").unwrap(), ["echo", "premidpost"]);
    }
}
