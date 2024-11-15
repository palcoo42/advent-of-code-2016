use std::{error::Error, fmt::Display};

/// Describes error codes which may occur during text file reading and analyses
#[derive(Debug)]
pub enum TextReaderError {
    OpenFileError(String, std::io::Error), // file_name, IO error
    FileReadError(String, std::io::Error), // file_name, IO error
    EmptyFileError(String),                // file_name
    InvalidContentError(String),           // description
    GenericError(String),                  // description
}

impl Error for TextReaderError {}

impl Display for TextReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextReaderError::OpenFileError(file_name, err) => {
                write!(
                    f,
                    "Failed to open file '{}' with error '{}'",
                    file_name, err
                )
            }
            TextReaderError::FileReadError(file_name, err) => {
                write!(
                    f,
                    "Failed to read from the file '{}' with error '{}'",
                    file_name, err
                )
            }
            TextReaderError::EmptyFileError(file_name) => {
                write!(f, "File '{}' is empty", file_name)
            }
            TextReaderError::InvalidContentError(desc) => {
                write!(f, "Invalid file content: {}", desc)
            }
            TextReaderError::GenericError(desc) => {
                write!(f, "{}", desc)
            }
        }
    }
}
