use aoc::solutions;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run -- <day>");
        std::process::exit(1);
    }
    solutions::run(&args[1]);
}
