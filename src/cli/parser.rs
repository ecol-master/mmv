//! Command line arguments parser. Holds the `Args` struct that is used to parse the command line arguments.
use clap::Parser;

/// MassMove (mmv) is a command line tool that renames files and directories in a given directory.
/// This tool is useful when you want to rename multiple files and directories in a directory.
/// Use the `--force` flag to overwrite existing files and directories.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    force: bool,
    source_path: String,
    target_path: String,
}

impl Args {
    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn target_path(&self) -> &str {
        &self.target_path
    }

    pub fn force(&self) -> bool {
        self.force
    }
}
