use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to source code file
    file_path: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let text = fs::read(args.file_path)?;

    println!("{}", char::from_u32(text[0] as u32).unwrap());

    Ok(())
}
