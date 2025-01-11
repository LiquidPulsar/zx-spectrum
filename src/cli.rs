use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Args {
    pub path: std::path::PathBuf,
    #[clap(long, short, action)]
    pub prefixed: bool,
}