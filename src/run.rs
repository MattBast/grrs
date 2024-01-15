use anyhow::{Context, Result};
use std::io::{BufRead};

/// Search a files contents for the desired `pattern`.
pub fn search_file<R: BufRead>(
    reader: R, 
    pattern: &str, 
    mut writer: impl std::io::Write
) -> Result<()> {
    
    for line in reader.lines() {
        
        // read the line
        let line_content = line
            .with_context(|| "Could not read line of file.".to_string())?;
        
        // check if the line contains the `pattern
        if line_content.contains(pattern) {
            // print the line if it does
            writeln!(writer, "{}", &line_content)
                .with_context(|| "Unable to write line to writer.".to_string())?;
        }

    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::fs::File;
    use assert_fs::prelude::*;
    use proptest::prelude::*;

    // ************************************************************
    // unit tests
    // ************************************************************

    #[test]
    fn search_file_for_pattern() -> Result<()> {
        
        // generate a test file that deletes itself at the end of the test
        let file = assert_fs::NamedTempFile::new("hello.txt")?;
        let _ = file.write_str("Hello world!\n");

        let reader = BufReader::new(File::open(file.path())?);
        let mut writer = Vec::new();

        let result = search_file(reader, "Hello", &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"Hello world!\n");

        Ok(())

    }

    // ************************************************************
    // property tests
    // ************************************************************

    // Randomly pass strings that don't contain control characters to the
    // function to make sure it doesn't panic with rare text.
    proptest! {
        #[test]
        fn search_file_doesnt_crash(s in "\\PC*") {
            
            // generate a test file that deletes itself at the end of the test
            let file = assert_fs::NamedTempFile::new("hello.txt")?;
            let _ = file.write_str(&s);

            let reader = BufReader::new(File::open(file.path())?);
            let mut writer = Vec::new();

            let _ = search_file(reader, &s, &mut writer);

        }
    }

    // Generate three random strings, merge them together and make sure the function
    // can still find the one marked as the pattern.
    proptest! {
        #[test]
        fn search_file_always_finds_matching_text(
            pattern in "\\PC*",
            prefix in "\\PC*",
            suffix in "\\PC*"
        ) {
            
            let file_content = format!("{}{}{}", prefix, pattern, suffix);

            // generate a test file that deletes itself at the end of the test
            let file = assert_fs::NamedTempFile::new("hello.txt")?;
            let _ = file.write_str(&file_content);

            let reader = BufReader::new(File::open(file.path())?);
            let mut writer = Vec::new();

            let result = search_file(reader, &pattern, &mut writer);

            prop_assert_eq!(result.is_ok(), true);
            
            // Converts writer (bytes) into a string, adds a newline character 
            // to file content and then compares the two.
            prop_assert_eq!(
                String::from_utf8(writer).unwrap(), 
                format!("{}\n", file_content)
            );

        }
    }
}