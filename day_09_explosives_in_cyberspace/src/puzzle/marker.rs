use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Marker {
    begin_index: usize,
    end_index: usize,
    nr_of_chars: usize,
    repetitions: usize,
}

impl Marker {
    pub fn find_marker(data: &str, index: usize) -> Result<Option<Marker>, PuzzleError> {
        match data[index..].find("(") {
            Some(begin) => match data[index..].find(")") {
                Some(end) => {
                    let begin_index = index + begin;
                    let end_index = index + end + 1; // Include ending ")"

                    Self::decode_marker(data, begin_index, end_index).map(Some)
                }
                None => Err(PuzzleError::GenericError(format!(
                    "Could not find marker end in '{}'",
                    &data[begin..],
                ))),
            },
            None => Ok(None),
        }
    }

    fn decode_marker(
        marker: &str,
        begin_index: usize,
        end_index: usize,
    ) -> Result<Marker, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^\((\w+)x(\w+)\)").expect("Failed to create 'marker' regex")
        });

        // Analyze only begin_index..end_index, these contains exactly marker
        let marker = &marker[begin_index..end_index];

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

            return Ok(Marker {
                begin_index,
                end_index,
                nr_of_chars,
                repetitions,
            });
        }

        Err(PuzzleError::GenericError(format!(
            "Failed to decode marker '{}'",
            marker,
        )))
    }

    pub fn get_nr_of_chars(&self) -> usize {
        self.nr_of_chars
    }

    pub fn get_repetitions(&self) -> usize {
        self.repetitions
    }

    pub fn get_begin_index(&self) -> usize {
        self.begin_index
    }

    pub fn get_end_index(&self) -> usize {
        self.end_index
    }

    pub fn get_length(&self) -> usize {
        self.end_index - self.begin_index
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::marker::Marker;

    #[test]
    fn test_decode_marker() {
        let marker_str = "(42x24)";
        let result = Marker::decode_marker(marker_str, 0, marker_str.len());

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Marker {
                begin_index: 0,
                end_index: 7,
                nr_of_chars: 42,
                repetitions: 24,
            }
        );
    }
}
