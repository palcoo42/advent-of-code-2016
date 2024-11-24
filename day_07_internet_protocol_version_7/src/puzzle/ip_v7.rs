use core::panic;

const ABBA_LENGTH: usize = 4;

pub struct Ipv7 {
    address: String,
}

impl Ipv7 {
    pub fn new(address: &str) -> Self {
        Self {
            address: String::from(address),
        }
    }

    pub fn is_tls_supported(&self) -> bool {
        let mut inside_brackets = false;
        let mut valid_abba = false;
        let mut invalid_abba = false;

        for i in 0..=self.address.len() - ABBA_LENGTH {
            let first = self.get_char(i);

            match first {
                '[' => {
                    inside_brackets = true;
                    continue;
                }
                ']' => {
                    inside_brackets = false;
                    continue;
                }
                _ => {}
            }

            if self.is_abba(i) {
                match inside_brackets {
                    false => {
                        valid_abba = true;
                    }
                    true => {
                        invalid_abba = true;
                    }
                }
            }

            // If we have invalid ABBA pattern we know that IPv7 does not support TLS
            if invalid_abba {
                return false;
            }
        }

        valid_abba
    }

    fn get_char(&self, n: usize) -> char {
        self.address
            .chars()
            .nth(n)
            .unwrap_or_else(|| panic!("Failed to access element '{}' in '{}'", n, self.address))
    }

    fn is_abba(&self, index: usize) -> bool {
        self.get_char(index) == self.get_char(index + 3)
            && self.get_char(index + 1) == self.get_char(index + 2)
            && self.get_char(index) != self.get_char(index + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abba() {
        let ipv7 = Ipv7::new("abba[bbbb]qrst");

        assert!(ipv7.is_abba(0));

        assert!(!ipv7.is_abba(1));
        assert!(!ipv7.is_abba(2));
        assert!(!ipv7.is_abba(3));
        assert!(!ipv7.is_abba(4));
        assert!(!ipv7.is_abba(5));
        assert!(!ipv7.is_abba(6));
        assert!(!ipv7.is_abba(7));
        assert!(!ipv7.is_abba(8));
    }

    #[test]
    fn test_is_tls_supported() {
        assert!(Ipv7::new("abba[mnop]qrst").is_tls_supported());
        assert!(Ipv7::new("ioxxoj[asdfgh]zxcvbn").is_tls_supported());
        assert!(Ipv7::new("aaaa[mnop]abba").is_tls_supported());

        assert!(!Ipv7::new("abcd[bddb]xyyx").is_tls_supported());
        assert!(!Ipv7::new("aaaa[qwer]tyui").is_tls_supported());
        assert!(!Ipv7::new("aaaa[xyyx]abba").is_tls_supported());
    }
}
