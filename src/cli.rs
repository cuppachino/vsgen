use std::path::PathBuf;

use clap::Parser;

/// Command line arguments for the application
#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
pub struct Args {
    /// Comma-separated paths to input files or directories to process
    #[arg(short = 'i', long = "input", value_parser)]
    paths: Vec<PathBuf>,

    /// Preview program output without making any changes to the file system.
    #[arg(short = None, long = "dry-run")]
    is_dry_run: bool,

    #[arg(short, long, default_value = "dist")]
    dist: String,
}

impl Args {
    /// Returns the paths to the input files or directories to process.
    #[inline]
    pub fn paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }

    /// Returns whether the program is in dry run mode.
    #[inline]
    pub fn is_dry_run(&self) -> bool {
        self.is_dry_run
    }

    #[inline]
    pub fn dist(&self) -> &str {
        self.dist.as_str()
    }
}
