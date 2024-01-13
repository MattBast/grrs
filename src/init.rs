use anyhow::{Context, Result, anyhow};
use clap::Parser;
use log::{info, error};
use std::path::PathBuf;
use std::io::{BufReader};
use std::fs::File;
use std::process::exit;

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

/// Read the user inputs, check they're valid and create a BufReader
/// for the provided file.
pub fn init(args: &Cli) -> Result<BufReader<File>> {

    match check_pattern(&args.pattern) {
        Ok(()) => (),
        Err(e) => {
            error!("{:?}", e);
            exit(exitcode::DATAERR);
        }
    };

    // Read the files contents.
    match open_file(&args.path) {
        Ok(reader) => {
            info!("Opened the file {}", &args.path.display());
            Ok(reader)
        },
        Err(e) => {
            error!("{:?}", e);
            exit(exitcode::NOINPUT)
        }
    }

}

/// Make sure the pattern provided is not empty
fn check_pattern(pattern: &String) -> Result<()> {

    if pattern.is_empty() {
        return Err(anyhow!("The pattern provided is an empty string."));
    }

    Ok(())
}

// Read a file into a buffer and return a reader that can be used
// to read the lines of the file later.
fn open_file(path: &PathBuf) -> Result<BufReader<File>> {

    // Try to open the file.
    let f = File::open(path)
        .with_context(|| format!("Could not open the file `{}`", path.display()))?;
    
    // Load it into a Buffer so it can be read line by line without needing to 
    // open the whole thing at once.
    let reader = BufReader::new(f);

    Ok(reader)

}