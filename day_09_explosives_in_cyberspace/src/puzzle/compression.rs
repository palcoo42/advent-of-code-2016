use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::marker::Marker;

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
                let marker =
                    Marker::find_marker(data, idx)?.unwrap_or_else(|| panic!("Marker not found"));

                let marker_end = idx + marker.get_length();
                let next = marker_end + marker.get_nr_of_chars();

                // Copy data
                let copied = data[marker_end..next].repeat(marker.get_repetitions());
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

    pub fn decompress_v2(data: &str) -> Result<usize, PuzzleError> {
        let mut weights = vec![1; data.len()];
        let mut idx = 0;

        while idx < data.len() {
            if &data[idx..=idx] == "(" {
                // We have found a marker
                let marker = Marker::find_marker(data, idx)?.unwrap_or_else(|| {
                    panic!("Marker not found '{}'", &data[idx..]);
                });

                let start = marker.get_end_index();
                let end = start + marker.get_nr_of_chars();

                // Set all marker places with 0 so we can easily sum up below
                weights
                    .iter_mut()
                    .take(start)
                    .skip(idx)
                    .for_each(|w| *w = 0);

                // Update weights for markers
                weights
                    .iter_mut()
                    .take(end)
                    .skip(start)
                    .for_each(|w| *w *= marker.get_repetitions());

                // Update current index at the end of the marker
                idx += marker.get_length();
            } else {
                // For non-marker values we have already counted in initial vector
                idx += 1;
            }
        }

        // Count result
        Ok(weights.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        let result = Compression::decompress("ADVENT");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), "ADVENT");

        let result = Compression::decompress("A(1x5)BC");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), "ABBBBBC");

        let result = Compression::decompress("(3x3)XYZ");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), "XYZXYZXYZ");

        let result = Compression::decompress("A(2x2)BCD(2x2)EFG");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), "ABCBCDEFEFG");

        let result = Compression::decompress("(6x1)(1x3)A");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), "(1x3)A");

        let result = Compression::decompress("X(8x2)(3x3)ABCY");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_decompress_v2() {
        let result = Compression::decompress_v2("(3x3)XYZ");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), 9);

        let result = Compression::decompress_v2("X(8x2)(3x3)ABCY");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), 20);

        let result = Compression::decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), 241920);

        let result =
            Compression::decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN");
        assert!(result.is_ok(), "Failed: {:?}", result);
        assert_eq!(result.unwrap(), 445);
    }
}
