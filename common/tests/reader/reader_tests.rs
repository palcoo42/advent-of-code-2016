use std::path::Path;

use common::{
    env::project::Project,
    reader::{text_reader::TextReader, text_reader_error::TextReaderError},
};

use super::simple_parser::SimpleParser;

#[test]
fn test_reader() {
    let parser = SimpleParser::new();
    let reader = TextReader::new(parser);

    let input_file = Project::new().resource_test_file("input.txt");

    let result = reader.read_with_hint(&input_file, 5);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_reader_not_existing_file() {
    let parser = SimpleParser::new();
    let reader = TextReader::new(parser);

    let result = reader.read_with_hint(Path::new("not_existing_path"), 1);

    assert!(result.is_err());
    assert!(
        matches!(result, Err(TextReaderError::OpenFileError(_, _)),),
        "result: {:?}",
        result
    )
}

#[test]
fn test_reader_empty_file() {
    let parser = SimpleParser::new();
    let reader = TextReader::new(parser);

    let input_file = Project::new().resource_test_file("empty_file.txt");

    let result = reader.read_with_hint(&input_file, 5);

    assert!(result.is_err());
    assert!(
        matches!(result, Err(TextReaderError::EmptyFileError(_))),
        "result: {:?}",
        result
    );
}

#[test]
fn test_reader_invalid_content() {
    let parser = SimpleParser::new();
    let reader = TextReader::new(parser);

    let input_file = Project::new().resource_test_file("input_invalid.txt");

    let result = reader.read_with_hint(&input_file, 5);

    assert!(result.is_err());
    assert!(
        matches!(result, Err(TextReaderError::InvalidContentError(_))),
        "result: {:?}",
        result
    );
}
