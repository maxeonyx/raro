use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use chumsky::Parser as ChumParser;
use clap::Parser as ClapParser;

mod parser;

// Add two numbers.
fn run(file: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    // Read file to string
    let contents = fs::read_to_string(file)?;

    println!("With text:\n===\n{}\n===\n", contents);

    println!("{:?}", parser::file_parser().parse(&contents).unwrap());

    Ok(())
}

/// RARO - Running ARithmetic with Objects
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args.file) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
