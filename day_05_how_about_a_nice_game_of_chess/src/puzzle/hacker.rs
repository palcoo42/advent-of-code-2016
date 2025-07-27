const PASSWORD_LENGTH: usize = 8;

pub struct Hacker {
    door_id: String,
}

impl Hacker {
    pub fn new(door_id: &str) -> Self {
        Self {
            door_id: door_id.to_string(),
        }
    }

    pub fn crack_password(&self) -> String {
        let mut pwd = String::with_capacity(PASSWORD_LENGTH);
        let mut buffer: Vec<u8> = Vec::with_capacity(self.door_id.len() + 12);
        let mut number: usize = 1;

        while pwd.len() != PASSWORD_LENGTH {
            // Prepare buffer
            buffer.clear();
            buffer.extend_from_slice(self.door_id.as_bytes());
            buffer.extend_from_slice(number.to_string().as_bytes());

            // Calculate hash
            let digest = md5::compute(&buffer);

            // Find five zeros
            // Digest is [u8; 16]
            // byte 0 -> two zeros
            // byte 1 -> two zeros
            // byte 2 -> one zero (higher byte)
            let five_zeros =
                digest.0[0] == 0 && digest.0[1] == 0 && ((digest.0[2] & 0xF0) >> 4) == 0;

            if five_zeros {
                // 6th number is lower part of 3rd byte
                let code = digest.0[2] & 0x0F;
                let code = format!("{code:x}");
                pwd.push_str(&code);
            }

            number += 1;
        }

        pwd
    }

    pub fn crack_password_advanced(&self) -> String {
        // _ means no pwd yet in the position
        const EMPTY: char = '_';
        let mut pwd = [EMPTY; PASSWORD_LENGTH];
        let mut buffer: Vec<u8> = Vec::with_capacity(self.door_id.len() + 12);
        let mut number: usize = 1;

        while pwd.iter().any(|c| c == &EMPTY) {
            // Prepare buffer
            buffer.clear();
            buffer.extend_from_slice(self.door_id.as_bytes());
            buffer.extend_from_slice(number.to_string().as_bytes());

            // Calculate hash
            let digest = md5::compute(&buffer);

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

                // 7th number is higher part of 4th byte
                let code = (digest.0[3] & 0xF0) >> 4;
                let code = format!("{code:x}");

                // Write only on first occurence + skip invalid position
                if position < PASSWORD_LENGTH && pwd[position] == EMPTY {
                    pwd[position] = code
                        .chars()
                        .next()
                        .expect("Code shall contain single hex character");
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
}
