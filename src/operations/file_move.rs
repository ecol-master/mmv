use crate::{config::Config, errors::MassMoveError};
use std::fs;
use std::path::PathBuf;

/// Public struct that represents a pair of files to move.
pub struct MoveFiles {
    pub from: String,
    pub to: String,
}

/// FilesMover is a struct that moves files from one location to another.
/// It takes a `Config` struct and a vector of `MoveFiles` structs.
/// Usage:
/// ```rust
/// use crate::file_move::{FilesMover, MoveFiles};
/// use crate::config::Config;
/// ...
/// let config = Config::new();
/// let files_to_move = FileMatcher(...)
///
/// let result: Result<(), MassMoveError> = FilesMover::new(config, files_to_move).run();
/// ```
///
/// Method run() drop the struct because we don't need to use files after moving
pub struct FilesMover {
    config: Config,
    files_to_move: Vec<MoveFiles>,
}

impl FilesMover {
    /// Create a new FilesMover struct from config and files to move
    pub fn new(config: Config, files_to_move: Vec<MoveFiles>) -> Self {
        FilesMover {
            config,
            files_to_move,
        }
    }

    /// Function that checks if the target path is valid.
    /// Usage: `correct_target_path("path/to/file")?`
    fn correct_target_path(&self, target_path: &str) -> Result<(), MassMoveError> {
        let path = PathBuf::from(target_path);
        let parent = path.parent();

        if parent.is_none() {
            return Err(MassMoveError::InvalidTargetPath(target_path.to_owned()));
        }

        let is_empty_parent = parent.unwrap().to_str().unwrap().is_empty();
        if !is_empty_parent && !parent.unwrap().exists() {
            return Err(MassMoveError::DirectoryNotFound(
                parent.unwrap().to_str().unwrap().to_owned(),
            ));
        }

        if !self.config.force_move() && path.exists() {
            return Err(MassMoveError::FileAlreadyExists(String::from(target_path)));
        }

        Ok(())
    }

    /// Function that move a concrete file from one location to another.
    fn move_file(&self, from: String, to: String) -> Result<(), MassMoveError> {
        self.correct_target_path(&to)?;

        match fs::rename(&from, &to) {
            Ok(_) => {
                println!("{} -> {}", from, to);
                Ok(())
            }
            Err(_) => Err(MassMoveError::MoveError(String::from(&from))),
        }
    }

    /// Function that moves all files from the vector of MoveFiles.
    /// It drops the struct after moving files.
    pub fn run(self) -> Result<(), MassMoveError> {
        for file_pair in &self.files_to_move {
            self.move_file(file_pair.from.to_owned(), file_pair.to.to_owned())?;
        }
        Ok(())
    }
}
