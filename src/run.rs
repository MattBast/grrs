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
            .with_context(|| format!("Could not read line of file."))?;
        
        // check if the line contains the `pattern
        if line_content.contains(pattern) {
            // print the line if it does
            writeln!(writer, "{}", &line_content)
                .with_context(|| format!("Unable to write line to writer."))?;
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
}