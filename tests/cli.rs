use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::prelude::*;

#[test]
fn can_find_content_in_file() -> Result<()> {
	
	// generate a test file that deletes itself at the end of the test
	let file = assert_fs::NamedTempFile::new("hello.txt")?;
	let _ = file.write_str("Hello world!\n");

	// load the main function in the binary file
	let mut cmd = Command::cargo_bin("grrs")?;

	// call the function from the command line looking for
	// the word "Hello" in the file "hello.txt".
	cmd.arg("Hello").arg(file.path());

	// make sure the function returns a stderr
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Hello world!"));

	Ok(())
	
}

#[test]
fn path_pointing_at_non_existant_file_returns_error() -> Result<()> {
	
	// load the main function in the binary file
	let mut cmd = Command::cargo_bin("grrs")?;

	// call the function from the command line looking for
	// the word "Hello" in a non-existent file.
	cmd.arg("Hello").arg("./hey.txt");

	// make sure the function returns a stderr
	cmd.assert()
		.failure()
		.stderr(predicate::str::contains("Could not open the file `./hey.txt`"));

	Ok(())

}

#[test]
fn empty_string_pattern_returns_error() -> Result<()> {
	
	// generate a test file that deletes itself at the end of the test
	let file = assert_fs::NamedTempFile::new("hello.txt")?;

	// load the main function in the binary file
	let mut cmd = Command::cargo_bin("grrs")?;

	// call the function from the command line looking for
	// the word "Hello" in a non-existent file.
	cmd.arg("").arg(file.path());

	// make sure the function returns a stderr
	cmd.assert()
		.failure()
		.stderr(predicate::str::contains("The pattern provided is an empty string."));

	Ok(())

}