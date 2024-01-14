use anyhow::{Result};
use clap::Parser;
use std::io::stdout;
use std::process::exit;


fn main() -> Result<()> {
    // Get the args from the command line.
    let args = grrs::args::Cli::parse();

    // Read user inputs and make sure they're valid. Return
    // a reader that can read the provided file.
    let reader = grrs::init::init(&args)?;

    // Search the file contents for the desired `pattern`.
    grrs::search_file(reader, &args.pattern, &mut stdout())?;
    exit(exitcode::OK);
}