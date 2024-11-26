use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

use crate::puzzle::commands::{
    rectangle::Rectangle, rotate_column::RotateColumn, rotate_row::RotateRow,
};

use super::commands::command::Command;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Vec<Box<dyn Command>>, PuzzleError> {
        lines.iter().map(|line| Self::parse_line(line)).collect()
    }

    fn parse_line(line: &str) -> Result<Box<dyn Command>, PuzzleError> {
        // Find which parser we need
        let splitted = line.split_ascii_whitespace().collect::<Vec<_>>();

        if splitted.is_empty() {
            return Err(PuzzleError::InvalidContentError(format!(
                "Splitted line '{}' is empty",
                line
            )));
        }

        match splitted[0] {
            "rect" => Self::decode_rectangle(line),
            "rotate" => Self::decode_rotate(line),
            _ => Err(PuzzleError::InvalidContentError(format!(
                "Failed to parse line '{}'",
                line
            ))),
        }
    }

    fn decode_rectangle(line: &str) -> Result<Box<dyn Command>, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^rect (\w+)x(\w+)").expect("Failed to create 'rectangle' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let width = captures[1].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse width '{}' to usize with error '{}'",
                    &captures[1], err
                ))
            })?;

            let height = captures[2].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse height '{}' to usize with error '{}'",
                    &captures[2], err
                ))
            })?;

            return Ok(Box::new(Rectangle::new(width, height)));
        }

        Err(PuzzleError::InvalidContentError(format!(
            "Failed to decode line '{}' to rectangle",
            line
        )))
    }

    fn decode_rotate(line: &str) -> Result<Box<dyn Command>, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^rotate (\w+) [xy]=(\w+) by (\w+)")
                .expect("Failed to create 'rotate' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let rotation = &captures[1];
            let idx = captures[2].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse idx '{}' to usize with error '{}'",
                    &captures[2], err
                ))
            })?;
            let shift = captures[3].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse shift '{}' to usize with error '{}'",
                    &captures[3], err
                ))
            })?;

            return match rotation {
                "row" => Ok(Box::new(RotateRow::new(idx, shift)) as Box<dyn Command>),
                "column" => Ok(Box::new(RotateColumn::new(idx, shift)) as Box<dyn Command>),
                _ => Err(PuzzleError::InvalidContentError(format!(
                    "Failed to parse rotate command, invalid subcommand '{}', line: {}",
                    rotation, line,
                ))),
            };
        }

        Err(PuzzleError::InvalidContentError(format!(
            "Failed to decode line '{}' to rotate",
            line
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_rectangle() {
        let result = Parser::decode_rectangle("rect 42x24");
        assert!(result.is_ok(), "Result: {:?}", result);
    }

    #[test]
    fn test_decode_rotate_invalid() {
        let result = Parser::decode_rotate("");
        assert!(result.is_err(), "Result: {:?}", result);

        let result = Parser::decode_rotate("rotate uups y=42 by 24");
        assert!(result.is_err(), "Result: {:?}", result);
    }

    #[test]
    fn test_decode_rectangle_failed() {
        let result = Parser::decode_rectangle("");
        assert!(result.is_err(), "Result: {:?}", result);

        let result = Parser::decode_rectangle("uups 42x24");
        assert!(result.is_err(), "Result: {:?}", result);

        let result = Parser::decode_rectangle("uups AAx24");
        assert!(result.is_err(), "Result: {:?}", result);

        let result = Parser::decode_rectangle("uups 42xAA");
        assert!(result.is_err(), "Result: {:?}", result);
    }

    #[test]
    fn test_decode_rotate_row() {
        let result = Parser::decode_rotate("rotate row y=42 by 24");
        assert!(result.is_ok(), "Result: {:?}", result);
    }

    #[test]
    fn test_decode_rotate_row_failed() {
        let result = Parser::decode_rotate("rotate row y=AA by 24");
        assert!(result.is_err(), "Result: {:?}", result);

        let result = Parser::decode_rotate("rotate row y=42 by AA");
        assert!(result.is_err(), "Result: {:?}", result);
    }

    #[test]
    fn test_decode_rotate_column() {
        let result = Parser::decode_rotate("rotate column y=42 by 24");
        assert!(result.is_ok(), "Result: {:?}", result);
    }

    #[test]
    fn test_decode_rotate_column_failed() {
        let result = Parser::decode_rotate("rotate column y=AA by 24");
        assert!(result.is_err(), "Result: {:?}", result);

        let result = Parser::decode_rotate("rotate column y=42 by AA");
        assert!(result.is_err(), "Result: {:?}", result);
    }
}
