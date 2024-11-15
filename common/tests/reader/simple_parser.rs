use common::reader::{text_parser::TextParser, text_reader_error::TextReaderError};

#[derive(Default)]
pub struct SimpleParser {}

impl SimpleParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TextParser for SimpleParser {
    type ParsedType = Vec<u32>;

    fn consume_lines(&self, lines: Vec<String>) -> Result<Self::ParsedType, TextReaderError> {
        let mut numbers = Vec::new();

        for line in lines {
            let value = line.parse::<u32>().map_err(|err| {
                TextReaderError::InvalidContentError(format!(
                    "Failed to parse '{}' to u32 with error '{}'",
                    line, err
                ))
            })?;

            numbers.push(value);
        }

        Ok(numbers)
    }
}
