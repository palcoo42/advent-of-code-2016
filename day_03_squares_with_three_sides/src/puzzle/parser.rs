use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::triangle::Triangle;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Vec<Triangle>, PuzzleError> {
        let mut triangles = Vec::new();

        for line in lines {
            let triangle = Self::parse_triangle(line)?;
            triangles.push(triangle);
        }

        Ok(triangles)
    }

    pub fn parse_lines_vertically(lines: &[&str]) -> Result<Vec<Triangle>, PuzzleError> {
        // Verify that number of lines is divisible by 3, i.e. we can make valid triangles
        if lines.len() % 3 != 0 {
            return Err(PuzzleError::InvalidContentError(String::from("Cannot create triangle vertically [Number of lines in the input file is not divisible by 3]")));
        }

        // Create matrix with string values gathered vertically
        let mut numbers = Vec::new();

        for line in lines {
            let values = line.split_ascii_whitespace().collect::<Vec<_>>();
            if values.len() != 3 {
                return Err(PuzzleError::InvalidContentError(format!(
                    "Line '{line}' does not contain 3 elements"
                )));
            }

            numbers.push(values);
        }

        let mut triangles = Vec::new();

        // Create vertical triangles and parse them
        for cols in 0..3 {
            let mut row = 0;

            while row < numbers.len() {
                let raw_triangle = format!(
                    "{} {} {}",
                    numbers[row][cols],
                    numbers[row + 1][cols],
                    numbers[row + 2][cols]
                );
                row += 3;

                let triangle = Self::parse_triangle(&raw_triangle)?;
                triangles.push(triangle);
            }
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

        assert!(result.is_ok(), "Failed result: {:?}", result);
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

    #[test]
    fn test_parse_lines_vertically() {
        let lines = ["1 2 3", "4 5 6", "7 8 9"];

        let result = Parser::parse_lines_vertically(&lines);
        assert!(result.is_ok(), "Failed result: {:?}", result);

        let triangles = result.expect("Failed to unwrap result");

        assert_eq!(triangles.len(), 3);
        assert_eq!(triangles[0], Triangle::new(1, 4, 7));
        assert_eq!(triangles[1], Triangle::new(2, 5, 8));
        assert_eq!(triangles[2], Triangle::new(3, 6, 9));
    }
}
