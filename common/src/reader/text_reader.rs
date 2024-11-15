use std::{
    io::{BufRead, BufReader},
    path::Path,
};

use super::{text_parser::TextParser, text_reader_error::TextReaderError};

/// TextReader allows user to read the content of the text file.
///
/// The text content is read line by line and stored in vector of string values.
/// Extracted lines are afterward provided to the parser instance which implements TextParser trait.
/// Parser is responsible to decode lines of string values into parser specific structure.
pub struct TextReader<T>
where
    T: TextParser,
{
    parser: T,
}

impl<T> TextReader<T>
where
    T: TextParser,
{
    /// Creates new instance of the TextReader
    ///
    /// # Arguments
    ///
    /// _parser_ - Instance of the Parser which implements TextParser trait
    ///
    /// # Returns
    ///
    /// New instance of the TextReader
    ///
    pub fn new(parser: T) -> Self {
        Self { parser }
    }

    /// Reads the content of the text file. Retrieved lines are provided to the
    /// parser which decodes its content to TextParser::ParsedType
    ///
    /// # Arguments
    ///
    ///  _path_ - Path to the text file
    ///  _hint_ - Hint how many memory should be pre-allocated. Always whole file content is read
    ///           however if number of lines is larger than hint size additional reallocations may occur.
    ///
    /// # Returns
    ///
    /// Parsed type in case of success, or error code in case of a failure
    ///
    pub fn read_with_hint(
        &self,
        path: &Path,
        hint: usize,
    ) -> Result<<T as TextParser>::ParsedType, TextReaderError> {
        // Open file for reading
        let file = std::fs::File::open(path).map_err(|err| {
            TextReaderError::OpenFileError(
                path.to_str()
                    .expect("Failed to unwrap Path to file name")
                    .to_string(),
                err,
            )
        })?;

        // Create buffered reader for faster reading
        let reader = BufReader::new(file);

        // Allocate vector for lines
        let mut lines = Vec::with_capacity(hint);

        for line in reader.lines() {
            match line {
                Ok(l) => {
                    lines.push(l);
                }
                Err(err) => {
                    return Err(TextReaderError::FileReadError(
                        path.to_str()
                            .expect("Failed to unwrap Path to file name")
                            .to_string(),
                        err,
                    ));
                }
            }
        }

        if lines.is_empty() {
            return Err(TextReaderError::EmptyFileError(
                path.to_str()
                    .expect("Failed to unwrap Path to file name")
                    .to_string(),
            ));
        }

        self.parser.consume_lines(lines)
    }
}
