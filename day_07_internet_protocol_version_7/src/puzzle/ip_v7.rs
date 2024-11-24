const ABBA_LENGTH: usize = 4;
const ABA_LENGTH: usize = 3;

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
            let first = &self.address[i..=i];

            match first {
                "[" => {
                    inside_brackets = true;
                    continue;
                }
                "]" => {
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

    fn is_abba(&self, index: usize) -> bool {
        self.address[index..=index] == self.address[index + 3..=index + 3]
            && self.address[index + 1..=index + 1] == self.address[index + 2..=index + 2]
            && self.address[index..=index] != self.address[index + 1..=index + 1]
    }

    pub fn is_ssl_supported(&self) -> bool {
        let mut inside_brackets = false;
        let mut aba_collection = Vec::new();
        let mut bab_collection = Vec::new();

        for i in 0..=self.address.len() - ABA_LENGTH {
            let first = &self.address[i..=i];

            match first {
                "[" => {
                    inside_brackets = true;
                    continue;
                }
                "]" => {
                    inside_brackets = false;
                    continue;
                }
                _ => {}
            }

            if let Some(aba) = self.is_aba(i) {
                match inside_brackets {
                    false => {
                        aba_collection.push(aba);
                    }
                    true => {
                        bab_collection.push(aba);
                    }
                }
            }
        }

        // Check is we have the related ABA in BAB
        for aba in aba_collection {
            if bab_collection.iter().any(|bab| Self::is_related(aba, bab)) {
                return true;
            }
        }

        false
    }

    fn is_aba(&self, index: usize) -> Option<&str> {
        if self.address[index..=index] == self.address[index + 2..=index + 2]
            && self.address[index..=index] != self.address[index + 1..=index + 1]
        {
            return Some(&self.address[index..=index + 2]);
        }
        None
    }

    fn is_related(aba: &str, bab: &str) -> bool {
        aba[0..=0] == bab[1..=1] && aba[1..=1] == bab[0..=0]
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

    #[test]
    fn test_is_ssl_supported() {
        assert!(Ipv7::new("aba[bab]xyz").is_ssl_supported());
        assert!(Ipv7::new("aaa[kek]eke").is_ssl_supported());
        assert!(Ipv7::new("zazbz[bzb]cdb").is_ssl_supported());

        assert!(!Ipv7::new("xyx[xyx]xyx").is_ssl_supported());
    }

    #[test]
    fn is_related() {
        assert!(Ipv7::is_related("aba", "bab"));
        assert!(!Ipv7::is_related("aba", "xax"));
    }
}
