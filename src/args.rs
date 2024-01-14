use clap::Parser;
use std::path::PathBuf;

/// Search for a `pattern`in a file (located at `path`) and display the lines
/// that contain it.
#[derive(Debug, Parser)]
pub struct Cli {
    /// The pattern to look for in a file
    pub pattern: String,
    /// The path to the file to read
    pub path: PathBuf,
    /// Allow the user to pass the verbosity flag for printing error levels.
    /// Can pass -q (silence output), -v (show warnings), -vv (show info),
    /// -vvv (show debug) or -vvvv (show trace).
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}