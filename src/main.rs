use clap::Parser;
use std::time::SystemTime;
mod cli;

struct FileType {
    name: String,
    extension: String,
    size: String,
    modified: SystemTime,
    changed: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
    file_type: String,
}

fn main() -> std::io::Result<()> {
    let args = cli::Cli::parse();

    println!("{:?}", args);

    Ok(())
}
