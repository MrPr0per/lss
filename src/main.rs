use chrono::Local;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: lss <args...>");
        return;
    }
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    println!("{}{}{} {}", GREEN, now, RESET, args.join(" "));
}
