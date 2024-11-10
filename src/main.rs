mod cli;
mod config;
mod errors;
mod operations;
mod pattern;

use crate::errors::MassMoveError;
use clap::Parser;
use cli::parser::Args;
use config::Config;
use operations::file_matcher::FileMatcher;
use operations::file_move::{FilesMover, MoveFiles};
use pattern::insert_matches_in_target;
use std::path::PathBuf;
use std::process;

/// Function is wrapper for main function.
/// It help separate logic from main function. Moreove, it make easier to handling errors in main function.
fn run(args: Args, config: Config) -> Result<(), MassMoveError> {
    let matcher = FileMatcher::from_source_path(PathBuf::from(args.source_path()))?;
    let files_with_matches = matcher.get_files_with_matches()?;

    let mut files_to_move: Vec<MoveFiles> = Vec::new();
    for file_with_match in files_with_matches {
        let target_filepath =
            insert_matches_in_target(file_with_match.matches(), args.target_path())?;

        files_to_move.push(MoveFiles {
            from: file_with_match.filename().to_owned(),
            to: target_filepath.clone(),
        });
    }

    FilesMover::new(config, files_to_move).run()
}

fn main() {
    let args = Args::parse();
    let config = Config::from_args(&args);

    match run(args, config) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
