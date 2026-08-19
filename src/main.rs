fn main() {
    let mut shell = forge::Shell::new();

    if let Err(error) = shell.run() {
        eprintln!("forge: fatal error: {error}");
        std::process::exit(1);
    }
}
