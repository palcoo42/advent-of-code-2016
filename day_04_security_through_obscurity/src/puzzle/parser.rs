use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

use super::room::Room;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[String]) -> Result<Vec<Room>, PuzzleError> {
        lines.iter().map(|line| Self::line_to_room(line)).collect()
    }

    fn line_to_room(line: &str) -> Result<Room, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"^([-\w]+)-(\w+)\[(\w{5})\]$"#).expect("Failed to create regex for 'Room'")
        });

        if let Some(captures) = RE.captures(line) {
            let encrypted_name = &captures[1];
            let sector_id = captures[2].parse::<u32>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse sector ID '{}' to u32 with error '{}'",
                    &captures[2], err
                ))
            })?;
            let checksum = &captures[3];

            return Ok(Room::new(encrypted_name, sector_id, checksum));
        }

        Err(PuzzleError::InvalidContentError(format!(
            "Failed to decode Room from line '{line}'"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_room() {
        let result = Parser::line_to_room("aaaaa-bbb-z-y-x-123[abxyz]");

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), Room::new("aaaaa-bbb-z-y-x", 123, "abxyz"))
    }

    #[test]
    fn test_line_to_room_missing_sector_id() {
        assert!(matches!(
            Parser::line_to_room("aaaaa-bbb-z-y-x-XXX[abxyz]"),
            Err(PuzzleError::InvalidContentError(_))
        ));
    }

    #[test]
    fn test_line_to_room_missing_checksum() {
        assert!(matches!(
            Parser::line_to_room("aaaaa-bbb-z-y-x-123"),
            Err(PuzzleError::InvalidContentError(_))
        ));
    }

    #[test]
    fn test_line_to_room_short_checksum() {
        assert!(matches!(
            Parser::line_to_room("aaaaa-bbb-z-y-x-123[XXXX]"),
            Err(PuzzleError::InvalidContentError(_))
        ));
    }

    #[test]
    fn test_line_to_room_long_checksum() {
        assert!(matches!(
            Parser::line_to_room("aaaaa-bbb-z-y-x-123[XXXXXX]"),
            Err(PuzzleError::InvalidContentError(_))
        ));
    }
}
