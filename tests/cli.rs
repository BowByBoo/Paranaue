use std::process::Command;

fn forge() -> Command {
    Command::new(env!("CARGO_BIN_EXE_forge"))
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
