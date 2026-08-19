use std::process::{Command, Stdio};

fn forge() -> Command {
    Command::new(env!("CARGO_BIN_EXE_forge"))
}

fn run_with_stdin(input: &str) -> std::process::Output {
    let mut child = forge()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Forge binary should be runnable in integration tests");

    {
        use std::io::Write;
        let stdin = child.stdin.as_mut().expect("stdin should be piped");
        stdin
            .write_all(input.as_bytes())
            .expect("test input should be accepted");
    }

    child
        .wait_with_output()
        .expect("Forge process should terminate cleanly")
}

#[test]
fn version_flag_exits_successfully_and_reports_version() {
    let output = forge()
        .arg("--version")
        .output()
        .expect("Forge binary should be runnable in integration tests");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("Forge "));
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn short_version_flag_matches_long_version_flag() {
    let long = forge()
        .arg("--version")
        .output()
        .expect("Forge binary should be runnable");
    let short = forge()
        .arg("-V")
        .output()
        .expect("Forge binary should be runnable");

    assert!(long.status.success());
    assert!(short.status.success());
    assert_eq!(long.stdout, short.stdout);
}

#[test]
fn help_flag_exits_successfully_and_lists_core_commands() {
    let output = forge()
        .arg("--help")
        .output()
        .expect("Forge binary should be runnable in integration tests");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for command in ["help", "pwd", "cd <path>", "version", "exit"] {
        assert!(stdout.contains(command), "help output missing '{command}'");
    }
}

#[test]
fn short_help_flag_matches_long_help_flag() {
    let long = forge()
        .arg("--help")
        .output()
        .expect("Forge binary should be runnable");
    let short = forge()
        .arg("-h")
        .output()
        .expect("Forge binary should be runnable");

    assert!(long.status.success());
    assert!(short.status.success());
    assert_eq!(long.stdout, short.stdout);
}

#[test]
fn interactive_exit_terminates_cleanly_at_eof() {
    let output = run_with_stdin("exit\n");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Forge "));
}

#[test]
fn interactive_help_then_exit_reports_help_without_crashing() {
    let output = run_with_stdin("help\nexit\n");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Commands:"));
    assert!(stdout.contains("native process"));
}

#[test]
fn interactive_eof_exits_cleanly_without_an_exit_command() {
    let output = run_with_stdin("");
    assert!(output.status.success());
}
