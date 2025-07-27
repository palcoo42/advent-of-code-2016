const PASSWORD_LENGTH: usize = 8;
const HEX_DIGITS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];
const ASCII_DIGIT_0: u8 = 48;

pub struct Hacker {
    door_id: String,
}

impl Hacker {
    pub fn new(door_id: &str) -> Self {
        Self {
            door_id: door_id.to_string(),
        }
    }

    #[inline]
    fn number_to_u8(mut number: usize, array: &mut [u8; 8]) -> usize {
        let mut index = 0;

        loop {
            if number == 0 {
                break;
            }

            let code = (number % 10) as u8;
            number /= 10;

            array[index] = code + ASCII_DIGIT_0;
            index += 1;
        }

        array[..index].reverse();
        index
    }

    pub fn crack_password(&self) -> String {
        let mut pwd = String::with_capacity(PASSWORD_LENGTH);

        // Buffer for whole data to hash
        let mut buffer = [0_u8; 32];
        let mut buffer_len;

        // Buffer for number only
        let mut number: usize = 1;
        let mut buffer_number = [0_u8; 8];
        let mut buffer_number_len;

        // Insert door ir, it is same for all iterations
        buffer[..self.door_id.len()].copy_from_slice(self.door_id.as_bytes());

        while pwd.len() != PASSWORD_LENGTH {
            // Prepare buffer, door id is already filled
            // Convert manually number to chars
            buffer_number_len = Self::number_to_u8(number, &mut buffer_number);
            buffer_len = self.door_id.len() + buffer_number_len;

            buffer[self.door_id.len()..buffer_len]
                .copy_from_slice(&buffer_number[..buffer_number_len]);

            // Calculate hash
            let digest = md5::compute(&buffer[..buffer_len]);

            // Find five zeros
            // Digest is [u8; 16]
            // byte 0 -> two zeros
            // byte 1 -> two zeros
            // byte 2 -> one zero (higher byte)
            let five_zeros = digest.0[0] == 0 && digest.0[1] == 0 && (digest.0[2] & 0xF0) == 0;

            if five_zeros {
                // 6th number is lower part of 3rd byte
                let code = (digest.0[2] & 0x0F) as usize;
                let code = HEX_DIGITS[code];
                pwd.push(code);
            }

            number += 1;
        }

        pwd
    }

    pub fn crack_password_advanced(&self) -> String {
        // _ means no pwd yet in the position
        const EMPTY: char = '_';
        let mut pwd = [EMPTY; PASSWORD_LENGTH];

        // Buffer for whole data to hash
        let mut buffer = [0_u8; 32];
        let mut buffer_len;

        // Buffer for number only
        let mut number: usize = 1;
        let mut buffer_number = [0_u8; 8];
        let mut buffer_number_len;

        // Insert door ir, it is same for all iterations
        buffer[..self.door_id.len()].copy_from_slice(self.door_id.as_bytes());

        while pwd.iter().any(|c| c == &EMPTY) {
            // Prepare buffer, door id is already filled
            // Convert manually number to chars
            buffer_number_len = Self::number_to_u8(number, &mut buffer_number);
            buffer_len = self.door_id.len() + buffer_number_len;

            buffer[self.door_id.len()..buffer_len]
                .copy_from_slice(&buffer_number[..buffer_number_len]);

            // Calculate hash
            let digest = md5::compute(&buffer[..buffer_len]);

            // Find five zeros
            // Digest is [u8; 16]
            // byte 0 -> two zeros
            // byte 1 -> two zeros
            // byte 2 -> one zero (higher byte)
            let five_zeros =
                digest.0[0] == 0 && digest.0[1] == 0 && ((digest.0[2] & 0xF0) >> 4) == 0;

            if five_zeros {
                // 6th number is lower part of 3rd byte
                let position = (digest.0[2] & 0x0F) as usize;

                // Write only on first occurence + skip invalid position
                if position < PASSWORD_LENGTH && pwd[position] == EMPTY {
                    // 7th number is higher part of 4th byte
                    let code = ((digest.0[3] & 0xF0) >> 4) as usize;
                    let code = HEX_DIGITS[code];

                    pwd[position] = code;
                }
            }

            number += 1;
        }

        pwd.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crack_password() {
        assert_eq!(&Hacker::new("abc").crack_password(), "18f47a30");
    }

    #[test]
    fn test_crack_password_advanced() {
        assert_eq!(&Hacker::new("abc").crack_password_advanced(), "05ace8e3");
    }

    #[test]
    fn test_number_to_u8() {
        let mut array = [0_u8; 8];

        Hacker::number_to_u8(123, &mut array);
        assert_eq!(
            array,
            [
                1 + ASCII_DIGIT_0,
                2 + ASCII_DIGIT_0,
                3 + ASCII_DIGIT_0,
                0,
                0,
                0,
                0,
                0
            ]
        );
    }
}
