use std::fs;
use std::process::ExitCode;

use clap::Parser;

use algolc::regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to source code file
    file_path: String,
    pattern: String,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let text = fs::read_to_string(args.file_path).unwrap();

    if !args.pattern.is_ascii() || !text.is_ascii() {
        eprintln!("Non ascii text is not supported");
        return ExitCode::FAILURE;
    }

    let pattern = Regex::new(&args.pattern).unwrap();

    for line in text.lines() {
        if pattern.is_match(line) {
            println!("{}", line);
        }
    }
    return ExitCode::SUCCESS;
}
