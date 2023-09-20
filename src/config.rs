use std::path::PathBuf;

use clap::Parser;

/// Command line arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    /// Template file
    #[arg(short, long, value_name = "FILE")]
    pub template: PathBuf,
}

