use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser)]
    pub config: Option<PathBuf>,
}
