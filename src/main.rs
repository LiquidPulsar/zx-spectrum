use std::fs::read_to_string;
use anyhow::{anyhow, Error};
use clap::Parser;
mod cli;
mod exec;
mod parser;

fn main() -> Result<(), Error> {
    let args = cli::Args::parse();
    let content = read_to_string(args.path).map_err(|e| anyhow!("Failed to read file: {}", e))?;
    // If using map_err and ?, this won't compile: lifetime mismatch from desugared ? operator, so we'll go manual:
    let instrs = match parser::parse_file(&content, args.prefixed) {
        Ok(instrs) => instrs,
        Err(e) => return Err(anyhow!("Failed to parse file: {:#?}", e)),
    };

    exec::execute(instrs).map_err(|e| anyhow!("Failed to execute program: {:#?}", e))?;

    Ok(())
}
