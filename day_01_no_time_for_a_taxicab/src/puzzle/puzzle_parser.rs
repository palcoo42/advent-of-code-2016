use common::reader::{text_parser::TextParser, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use super::{action::Action, document::Document};

#[derive(Default)]
pub struct PuzzleParser {}

impl PuzzleParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TextParser for PuzzleParser {
    type ParsedType = Document;

    fn consume_lines(&self, lines: Vec<String>) -> Result<Self::ParsedType, TextReaderError> {
        let mut actions = Vec::with_capacity(lines.len());

        for line in lines {
            let mut line_actions = Self::parse_line(&line)?;
            actions.append(&mut line_actions);
        }

        Ok(Document::new(actions))
    }
}

impl PuzzleParser {
    fn parse_line(line: &str) -> Result<Vec<Action>, TextReaderError> {
        let instructions = line.split_terminator(", ").collect::<Vec<_>>();

        let mut actions = Vec::with_capacity(instructions.len());

        for instr in instructions {
            actions.push(Self::parse_action(instr)?);
        }

        Ok(actions)
    }

    fn parse_action(action: &str) -> Result<Action, TextReaderError> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"^([RL])(\w+)"#).expect("Failed to create regex 'Actions'"));

        if let Some(captures) = RE.captures(action) {
            let action_str = &captures[1];

            let steps = captures[2].parse::<u32>().map_err(|err| {
                TextReaderError::InvalidContentError(format!(
                    "Failed to convert steps '{}' to u32 with error '{}'",
                    &captures[2], err
                ))
            })?;

            return Ok(Action::new(action_str, steps));
        }

        Err(TextReaderError::InvalidContentError(format!(
            "Failed to decode action '{}'",
            action
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let parser = PuzzleParser::new();

        assert_eq!(
            parser
                .consume_lines(vec![String::from("L1, R3, R1, L5, L2, L5, R4")])
                .unwrap(),
            Document::new(vec![
                Action::Left(1),
                Action::Right(3),
                Action::Right(1),
                Action::Left(5),
                Action::Left(2),
                Action::Left(5),
                Action::Right(4)
            ])
        );
    }
}
