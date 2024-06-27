use clap::Parser;
use std::io;

#[derive(Parser, Debug, Clone)]
struct Cli {
    #[clap(short = 'E')]
    pattern: String,
}

fn matches_pattern(input: &str, pattern: String) -> bool {
    match pattern.as_str() {
        "\\d" => {
            for char in input.chars() {
                if char.is_numeric() {
                    return true;
                }
            }

            false
        }
        _ => return input.contains(&pattern),
    }
}

fn main() {
    let cli = Cli::parse();
    let pattern = cli.pattern;
    let mut input_line = String::new();
    println!("Pattern: {pattern}");

    io::stdin().read_line(&mut input_line).unwrap();
    if !matches_pattern(&input_line, pattern) {
        std::process::exit(1);
    }
}
