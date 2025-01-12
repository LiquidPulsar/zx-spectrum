use anyhow::{anyhow, Error};
use clap::Parser;
use std::fs::read_to_string;
mod cli;
mod exec;
mod parser;

fn main() -> Result<(), Error> {
    let args = cli::Args::parse();
    let content = read_to_string(args.path).map_err(|e| anyhow!("Failed to read file: {}", e))?;
    let instrs = parser::parse_file(&content, args.prefixed)
        .map_err(|e| anyhow!("Failed to parse file: {:#?}", e))?;

    exec::execute(instrs).map_err(|e| anyhow!("Failed to execute program: {:#?}", e))?;

    Ok(())
}
