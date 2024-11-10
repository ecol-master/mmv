//! Module that holds the Config struct that holds the configuration setting for the mmv.
use crate::cli::parser::Args;

/// Config is strcut that holds the configuration setting for the mmv.
/// There are only ways to create a Config struct:
/// 1. `Config::new()` - creates a new Config struct with default settings.
/// 2. `Config::from_args(args: &Args) - create a new Config from the command line arguments.`
///
/// Usage:
/// ```rust
/// use crate::config::Config;
/// use crate::cli::parser::Args;
///
/// let args = Args::parse();
/// let config = Config::from_args(&args);
/// ```
pub struct Config {
    force_move: bool,
}

impl Config {
    /// Construct a new Config struct with default settings.
    pub fn new() -> Self {
        Config { force_move: false }
    }

    /// Construct a new Config struct from the command line arguments.
    pub fn from_args(args: &Args) -> Self {
        Config {
            force_move: args.force(),
        }
    }

    pub fn force_move(&self) -> bool {
        self.force_move
    }
}
