use std::path::PathBuf;

use clap::Parser;

/// Cli args parsing
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Location of the config
    #[clap(short, long, value_parser)]
    pub config: Option<PathBuf>,
}
