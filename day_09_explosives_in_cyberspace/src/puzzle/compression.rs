use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

pub struct Compression {}

impl Compression {
    pub fn decompress(data: &str) -> Result<String, PuzzleError> {
        // Note: Magic constant 4 to avoid reallocations
        let mut decompressed = String::with_capacity(data.len() * 4);

        let mut idx = 0_usize;

        while idx < data.len() {
            let character = &data[idx..=idx];

            // Find marker for example: (5x6)
            if character == "(" {
                // Find end marker
                let marker = Self::locate_marker(&data[idx..])?;

                // Decode marker to know how to expand string
                let (nr_of_chars, repetitions) = Self::decode_marker(marker)?;

                let marker_end = idx + marker.len();
                let next = marker_end + nr_of_chars;

                // Copy data
                let copied = data[marker_end..next].repeat(repetitions);
                decompressed.push_str(&copied);

                idx = next;
            } else {
                // Simply copy character
                decompressed.push_str(character);
                idx += 1;
            }
        }

        Ok(decompressed)
    }

    fn locate_marker(data: &str) -> Result<&str, PuzzleError> {
        if &data[0..=0] != "(" {
            return Err(PuzzleError::GenericError(format!(
                "Cannot locate marker, first character is not ( '{}'",
                data
            )));
        }

        let marker_end = data.find(")").ok_or(PuzzleError::GenericError(format!(
            "Failed to find end marker ) '{}'",
            data
        )))?;

        Ok(&data[0..=marker_end])
    }

    // Decode marker and extract values
    //
    // # Returns
    //
    // Tuple (number of repeated character after marker, number of repetitions)
    fn decode_marker(marker: &str) -> Result<(usize, usize), PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^\((\w+)x(\w+)\)").expect("Failed to create 'marker' regex")
        });

        if let Some(captures) = RE.captures(marker) {
            let nr_of_chars = captures[1].parse::<usize>().map_err(|err| {
                PuzzleError::GenericError(format!(
                    "Failed to convert number of characters '{}' to usize with error '{}'",
                    &captures[1], err
                ))
            })?;

            let repetitions = captures[2].parse::<usize>().map_err(|err| {
                PuzzleError::GenericError(format!(
                    "Failed to convert repetitions '{}' to usize with error '{}'",
                    &captures[2], err
                ))
            })?;

            return Ok((nr_of_chars, repetitions));
        }

        Err(PuzzleError::GenericError(format!(
            "Failed to decode marker '{}'",
            marker,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        let result = Compression::decompress("ADVENT");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ADVENT");

        let result = Compression::decompress("A(1x5)BC");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ABBBBBC");

        let result = Compression::decompress("(3x3)XYZ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "XYZXYZXYZ");

        let result = Compression::decompress("A(2x2)BCD(2x2)EFG");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ABCBCDEFEFG");

        let result = Compression::decompress("(6x1)(1x3)A");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "(1x3)A");

        let result = Compression::decompress("X(8x2)(3x3)ABCY");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_decode_marker() {
        let result = Compression::decode_marker("(42x24)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (42, 24));
    }
}
