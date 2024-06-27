use clap::Parser;
use std::io;

mod engine;
use engine::RegexEngine;

#[derive(Parser, Debug, Clone)]
struct Cli {
    #[clap(short = 'E')]
    pattern: String,
}

fn main() {
    let cli = Cli::parse();
    let pattern = cli.pattern;
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let mut engine = RegexEngine::new(input_line, pattern);
    engine.matches();
}
