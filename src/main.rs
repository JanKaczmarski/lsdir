use clap::Parser;
mod cli;
mod file_type;
mod utilities;

use std::fs;



fn main() -> std::io::Result<()> {
    let args = cli::Cli::parse();

    println!("{:?}", args);

    Ok(())
}
