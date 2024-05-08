use std::{collections::HashMap, path::PathBuf};
use std::fs;
use std::error::Error;
use std::path::Path;

mod parser;

// Add two numbers.
fn run(file: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {

    // Read file to string
    let contents = fs::read_to_string(file)?;

    println!("With text:\n{}", contents);

    Ok(())
}

use clap::{Parser, Subcommand};

/// RARO - Running ARithmetic with Objects
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(value_name="FILE")]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args.file) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
