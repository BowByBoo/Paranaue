use std::env;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("--version") | Some("-V") | Some("version") => {
            println!("Forge {}", forge::VERSION);
            return;
        }
        Some("--help") | Some("-h") | Some("help") => {
            forge::print_help();
            return;
        }
        _ => {}
    }

    let mut shell = forge::Shell::new();

    if let Err(error) = shell.run() {
        eprintln!("forge: fatal error: {error}");
        std::process::exit(1);
    }
}
