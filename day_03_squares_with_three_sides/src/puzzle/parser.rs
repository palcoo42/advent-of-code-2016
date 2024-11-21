use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::triangle::Triangle;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: Vec<String>) -> Result<Vec<Triangle>, PuzzleError> {
        let mut triangles = Vec::new();

        for line in lines {
            let triangle = Self::parse_triangle(&line)?;
            triangles.push(triangle);
        }

        Ok(triangles)
    }

    fn parse_triangle(line: &str) -> Result<Triangle, PuzzleError> {
        let sides = line.split_ascii_whitespace().collect::<Vec<_>>();

        if sides.len() != 3 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Line '{line}' does not contain 3 numbers"
            )));
        }

        let a = sides[0].parse::<u32>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to convert '{}' to u32 with error '{}', line: '{}'",
                sides[0], err, line
            ))
        })?;

        let b = sides[1].parse::<u32>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to convert '{}' to u32 with error '{}', line: '{}'",
                sides[1], err, line
            ))
        })?;

        let c = sides[2].parse::<u32>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to convert '{}' to u32 with error '{}', line: '{}'",
                sides[2], err, line
            ))
        })?;

        Ok(Triangle::new(a, b, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_triangle() {
        let result = Parser::parse_triangle("1 2 3");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Triangle::new(1, 2, 3));
    }

    #[test]
    fn test_parse_triangle_number_of_sides_invalid() {
        let result = Parser::parse_triangle("");
        assert!(matches!(result, Err(PuzzleError::InvalidContentError(_))));

        let result = Parser::parse_triangle("1");
        assert!(matches!(result, Err(PuzzleError::InvalidContentError(_))));

        let result = Parser::parse_triangle("1 2");
        assert!(matches!(result, Err(PuzzleError::InvalidContentError(_))));

        let result = Parser::parse_triangle("1 2 3 4");
        assert!(matches!(result, Err(PuzzleError::InvalidContentError(_))));
    }

    #[test]
    fn test_parse_triangle_non_number() {
        let result = Parser::parse_triangle("1x 2 3");
        assert!(matches!(result, Err(PuzzleError::InvalidContentError(_))));

        let result = Parser::parse_triangle("1 x2 3");
        assert!(matches!(result, Err(PuzzleError::InvalidContentError(_))));

        let result = Parser::parse_triangle("1 2 3x");
        assert!(matches!(result, Err(PuzzleError::InvalidContentError(_))));
    }
}
