use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

use super::instruction::{Instruction, Recipient};

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Vec<Instruction>, PuzzleError> {
        lines
            .iter()
            .map(|&line| Self::decode_instruction(line))
            .collect()
    }

    fn decode_instruction(line: &str) -> Result<Instruction, PuzzleError> {
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();

        match split.first() {
            Some(&first) => match first {
                "value" => Self::decode_value(line),
                "bot" => Self::decode_assign(line),
                _ => Err(PuzzleError::InvalidContentError(format!(
                    "Unexpected first word '{}', line: '{}'",
                    first, line,
                ))),
            },
            None => Err(PuzzleError::InvalidContentError(format!(
                "Failed to split line: '{}'",
                line,
            ))),
        }
    }

    fn decode_value(line: &str) -> Result<Instruction, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^value (\w+) goes to bot (\w+)").expect("Failed to create 'value' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let value = captures[1].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse value '{}' to usize with an error '{}'",
                    &captures[1], err
                ))
            })?;

            let bot = captures[2].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse bot '{}' to usize with an error '{}'",
                    &captures[2], err
                ))
            })?;

            return Ok(Instruction::Assign(bot, value));
        }

        Err(PuzzleError::InvalidContentError(format!(
            "Failed to decode 'value', line: '{}'",
            line
        )))
    }

    fn decode_assign(line: &str) -> Result<Instruction, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^bot (\w+) gives low to (\w+) (\w+) and high to (\w+) (\w+)")
                .expect("Failed to create 'assign' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let bot = captures[1].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse bot '{}' to usize with an error '{}'",
                    &captures[1], err
                ))
            })?;

            let low_text = &captures[2];
            let low_id = captures[3].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse low id '{}' to usize with an error '{}'",
                    &captures[3], err
                ))
            })?;

            let low_recipient = Self::recipient(low_text, low_id)?;

            let high_text = &captures[4];
            let high_id = captures[5].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse high id '{}' to usize with an error '{}'",
                    &captures[5], err
                ))
            })?;

            let high_recipient = Self::recipient(high_text, high_id)?;

            return Ok(Instruction::Give(bot, low_recipient, high_recipient));
        }

        Err(PuzzleError::InvalidContentError(format!(
            "Failed to decode 'assign', line: '{}'",
            line
        )))
    }

    fn recipient(text: &str, id: usize) -> Result<Recipient, PuzzleError> {
        match text {
            "bot" => Ok(Recipient::Bot(id)),
            "output" => Ok(Recipient::Output(id)),
            _ => Err(PuzzleError::InvalidContentError(format!(
                "Unexpected recipient '{}'",
                text
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_value() {
        let result = Parser::decode_value("value 42 goes to bot 24");

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.as_ref().unwrap(), &Instruction::Assign(24, 42));
    }

    #[test]
    fn test_decode_recipient() {
        let result = Parser::decode_assign("bot 42 gives low to output 43 and high to bot 44");

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(
            result.as_ref().unwrap(),
            &Instruction::Give(42, Recipient::Output(43), Recipient::Bot(44))
        );
    }
}
