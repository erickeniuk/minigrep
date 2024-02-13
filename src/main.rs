use minigrep::Config;
use spinners::{Spinner, Spinners};
use std::env;
use std::process;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("\n");
    let mut sp = Spinner::new(
        Spinners::Dots9,
        "Hey Eric! Running your Rust program 0 _> o ...".into(),
    );
    sleep(Duration::from_secs(1));
    sp.stop();
    println!("\n");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for query: {}", config.query);
    println!("In file path: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
