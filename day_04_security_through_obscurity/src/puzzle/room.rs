use std::collections::HashMap;

#[derive(Debug)]
pub struct Room {
    name: String,
    sector_id: usize,
    checksum: String,
}

impl Room {
    pub fn new(name: &str, sector_id: usize, checksum: &str) -> Self {
        Self {
            name: name.to_string(),
            sector_id,
            checksum: checksum.to_string(),
        }
    }

    pub fn get_sector_id(&self) -> usize {
        self.sector_id
    }

    pub fn is_real(&self) -> bool {
        // Holds counts for all characters
        let mut counts: HashMap<char, usize> = HashMap::new();

        // Fill in counts for all words in the room name
        let words = self.name.split("-").collect::<Vec<_>>();

        for word in words {
            for c in word.chars() {
                let entry = counts.entry(c).or_insert(0);
                *entry += 1;
            }
        }

        // Sort items and crate full checksum
        let sorted = Self::sort_counts(&counts);
        let full_checksum = sorted.iter().map(|(c, _size)| *c).collect::<String>();

        full_checksum.starts_with(&self.checksum)
    }

    fn sort_counts(counts: &HashMap<char, usize>) -> Vec<(char, usize)> {
        let mut sorted = counts
            .iter()
            .map(|(c, size)| (*c, *size))
            .collect::<Vec<_>>();

        // Order by count, then alphabetically
        sorted.sort_by(|a, b| match a.1.cmp(&b.1) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                // Sort alphabetically
                a.0.cmp(&b.0)
            }
        });

        sorted
    }

    pub fn decode_name(&self) -> String {
        const LETTER_A: usize = 97;
        const LETTERS: usize = 26;

        self.name
            .chars()
            .map(|c| match c {
                '-' => ' ',
                c => {
                    (((c as usize - LETTER_A) + self.sector_id) % LETTERS + LETTER_A) as u8 as char
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_counts() {
        let counts: HashMap<char, usize> = [('b', 3), ('a', 5), ('z', 1), ('x', 1), ('y', 1)]
            .into_iter()
            .collect();

        assert_eq!(
            Room::sort_counts(&counts),
            vec![('a', 5), ('b', 3), ('x', 1), ('y', 1), ('z', 1)]
        );
    }

    #[test]
    fn test_is_real() {
        assert!(Room::new("aaaaa-bbb-z-y-x", 123, "abxyz").is_real());
        assert!(Room::new("a-b-c-d-e-f-g-h", 987, "abcde").is_real());
        assert!(Room::new("not-a-real-room", 404, "oarel").is_real());
    }

    #[test]
    fn test_is_real_not() {
        assert!(!Room::new("totally-real-room", 200, "decoy").is_real());
    }

    #[test]
    fn test_decode_name() {
        assert_eq!(
            Room::new("qzmt-zixmtkozy-ivhz", 343, "").decode_name(),
            String::from("very encrypted name")
        );
        assert_eq!(
            Room::new("ghkmaihex-hucxvm-lmhktzx", 267, "").decode_name(),
            String::from("northpole object storage")
        );
    }
}
