/// Module defines the base error type for the application - MassMoveError.
/// MassMoveError is an enum that represents all possible errors that can occur in the application.
/// It is used in all modules to return errors.
///
/// Usage (code example from file_matcher.rs):
/// ```rust
/// use crate::errors::MassMoveError;
/// use std::fs;
///
/// fn read_source_directory(&self) -> Result<ReadDir, MassMoveError> {
///    match fs::read_dir(&self.source_directory) {
///        Ok(read_directory) => Ok(read_directory),
///        Err(_) => Err(MassMoveError::DirectoryNotFound(
///            self.source_directory.clone(),
///        )),
///    }
/// }
/// ```
use std::{fmt::Display, io};

#[derive(Debug)]
pub enum MassMoveError {
    InvalidSourcePath(String),
    InvalidTargetPath(String),

    DirectoryNotFound(String),
    PermissionDenied(io::Error),

    NoFilesForPattern(String),

    FileAlreadyExists(String),
    MoveError(String),
    Error(io::Error),
}

/// Implementing From trait for io::Error to convert io::Error to MassMoveError.
impl From<io::Error> for MassMoveError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::PermissionDenied => MassMoveError::PermissionDenied(err),
            _ => MassMoveError::Error(err),
        }
    }
}

impl Display for MassMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MassMoveError::InvalidSourcePath(msg) => {
                write!(f, "mmv: Invalid source path: {}", msg)
            }
            MassMoveError::InvalidTargetPath(path) => {
                write!(f, "mmv: Invalid target path: {}", path)
            }
            MassMoveError::DirectoryNotFound(path) => {
                write!(f, "mmv: Directory `{}` no found", path)
            }
            MassMoveError::PermissionDenied(err) => {
                write!(f, "mmv: Permission denied: {}", err)
            }
            MassMoveError::FileAlreadyExists(path) => {
                write!(f, "mmv: Not able to replace existing file: {}", path)
            }
            MassMoveError::NoFilesForPattern(pattern) => {
                write!(f, "mmv: Files for pattern '{}' not found", pattern)
            }
            MassMoveError::MoveError(path) => {
                write!(f, "mmv: Failed move: {}", path)
            }
            MassMoveError::Error(err) => {
                write!(f, "mmv: {}", err)
            }
        }
    }
}
