use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: lss <name>");
        return;
    }
    println!("Hello, {}!", args[1]);
}
