use super::letter::Letters;

/// Length of the checksum
const CHECKSUM_LENGTH: usize = 5;

#[derive(Debug, PartialEq)]
pub struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    pub fn new(encrypted_name: &str, sector_id: u32, checksum: &str) -> Self {
        Self {
            encrypted_name: String::from(encrypted_name),
            sector_id,
            checksum: String::from(checksum),
        }
    }

    pub fn get_sector_id(&self) -> u32 {
        self.sector_id
    }

    pub fn is_real(&self) -> bool {
        let letters = self.count_letters();

        // Sort letters in descending order
        let mut sorted = letters.iter().collect::<Vec<_>>();

        sorted.sort_by(|a, b| {
            if a.1 == b.1 {
                // Counts are the same compare by char alphabetically
                a.0.cmp(b.0)
            } else {
                // Counts are different, sort from highest to lowest
                b.1.cmp(a.1)
            }
        });

        // Take first 5 characters to form a hash to format a hash
        if sorted.len() < CHECKSUM_LENGTH {
            panic!(
                "Encrypted name does not contain enough characters to format a checksum '{}'",
                self.encrypted_name
            );
        }

        let encrypted_checksum = format!(
            "{}{}{}{}{}",
            sorted[0].0, sorted[1].0, sorted[2].0, sorted[3].0, sorted[4].0
        );

        encrypted_checksum == self.checksum
    }

    fn count_letters(&self) -> Letters {
        let mut letters = Letters::new();

        for c in self.encrypted_name.chars() {
            // Skip all dashes
            if c == '-' {
                continue;
            }

            // Increment count
            let count = letters.entry(c).or_insert(0);
            *count += 1;
        }

        letters
    }

    pub fn decrypt(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|c| Self::shift_cipher(c, self.get_sector_id()))
            .collect::<String>()
    }

    fn shift_cipher(letter: char, loops: u32) -> char {
        match letter {
            '-' => ' ',
            'a'..='z' => Self::rotate_letter(letter, loops),
            invalid => panic!("Invalid cipher letter '{invalid}'"),
        }
    }

    fn rotate_letter(letter: char, loops: u32) -> char {
        const A: u32 = 'a' as u32;
        const Z: u32 = 'z' as u32;
        const LETTER_SIZE: u32 = Z - A + 1;

        let mut rotated = letter as u32 + loops % LETTER_SIZE;

        if rotated > Z {
            rotated = A + rotated - Z - 1;
        }

        char::from_u32(rotated).unwrap_or_else(|| panic!("Failed to convert '{rotated}' to u32"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_real() {
        assert!(Room::new("aaaaa-bbb-z-y-x", 123, "abxyz").is_real());
        assert!(Room::new("a-b-c-d-e-f-g-h", 987, "abcde").is_real());
        assert!(Room::new("not-a-real-room", 404, "oarel").is_real());
    }

    #[test]
    fn test_is_fake() {
        assert!(!Room::new("totally-real-room", 200, "decoy").is_real());
    }

    #[test]
    fn test_shift_cipher() {
        assert_eq!(Room::shift_cipher('-', 42), ' ');
        assert_eq!(Room::shift_cipher('a', 1), 'b');
        assert_eq!(Room::shift_cipher('b', 1), 'c');
        assert_eq!(Room::shift_cipher('z', 1), 'a');
        assert_eq!(Room::shift_cipher('z', 2), 'b');
        assert_eq!(Room::shift_cipher('q', 343), 'v');
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(
            Room::new("qzmt-zixmtkozy-ivhz", 343, "zimth").decrypt(),
            "very encrypted name"
        );
    }
}
