use std::process::exit;

use clap::Parser;
mod cli;
mod parser;
mod exec;

fn main() {
    let args = cli::Args::parse();
    let content = std::fs::read_to_string(args.path).expect("failed to read file");
    let res = parser::parse_file(&content, args.prefixed);

    let instrs = match res {
        Err(err) => {
            eprintln!("Error: {:#?}", err);
            exit(1);
        }
        Ok(instrs) => instrs
    };

    for instr in &instrs {
        println!("{:?}", instr);
    }

    if let Err(err) = exec::execute(instrs) {
        eprintln!("Error: failed to execute: {:#?}", err);
        exit(1);
    }
}
