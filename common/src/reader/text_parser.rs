use super::text_reader_error::TextReaderError;

/// Describes parser interface which can decode text file string lines into ParsedType
pub trait TextParser {
    /// Type of the parsed result
    type ParsedType;

    /// Consumes lines and decodes them to the ParsedType
    ///
    /// # Arguments
    ///
    /// _lines_ - Lines from text files
    ///
    /// # Returns
    ///
    /// Parsed type in case of success, or error code in case of a failure
    fn consume_lines(&self, lines: Vec<String>) -> Result<Self::ParsedType, TextReaderError>;
}
