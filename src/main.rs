use anyhow::{anyhow, Context, Error};
use clap::Parser;
use std::fs::read_to_string;
mod cli;
mod exec;
mod parser;

fn main() -> Result<(), Error> {
    let args = cli::Args::parse();
    let content = read_to_string(args.path).context("Failed to read file.")?;
    // This is very finnicky, .context("Failed to parse file.") fails to compile
    let instrs = parser::parse_file(&content, args.prefixed)
        .map_err(|e| anyhow!("Failed to parse file: {:#?}", e))?;

    exec::execute(instrs).context("Failed to execute program:")?;

    Ok(())
}
