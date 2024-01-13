use anyhow::{Context, Result, anyhow};
use log::{info, error};
use clap::Parser;
use std::path::PathBuf;
use std::io::{BufReader, stdout};
use std::fs::File;

/// Search for a `pattern`in a file (located at `path`) and display the lines
/// that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for in a file
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
    /// Allow the user to pass the verbosity flag for printing error levels.
    /// Can pass -q (silence output), -v (show warnings), -vv (show info),
    /// -vvv (show debug) or -vvvv (show trace).
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    // Get the args from the command line.
    let args = Cli::parse();

    check_pattern(&args.pattern)?;

    // Read the files contents.
    let reader = match open_file(&args.path) {
        Ok(reader) => {
            info!("Opened the file {}", &args.path.display());
            reader
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(e);
        }
    };

    // Search the file contents for the desired `pattern`.
    grrs::search_file(reader, &args.pattern, &mut stdout())
}

/// Make sure the pattern provided is not empty
fn check_pattern(pattern: &String) -> Result<()> {

    if pattern.is_empty() {
        return Err(anyhow!("The pattern provided is an empty string."))
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