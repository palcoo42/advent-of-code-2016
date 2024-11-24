#[derive(Debug)]
pub struct Signal {
    internal: String,
}

impl Signal {
    pub fn new(signal: &str) -> Self {
        Self {
            internal: String::from(signal),
        }
    }

    pub fn len(&self) -> usize {
        self.internal.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn nth(&self, n: usize) -> Option<char> {
        if n < self.internal.len() {
            return self.internal.chars().nth(n);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        assert_eq!(Signal::new("").len(), 0);
        assert_eq!(Signal::new("a").len(), 1);
        assert_eq!(Signal::new("ab").len(), 2);
        assert_eq!(Signal::new("abc").len(), 3);
    }

    #[test]
    fn test_is_empty() {
        assert!(Signal::new("").is_empty());
        assert!(!Signal::new("a").is_empty());
        assert!(!Signal::new("ab").is_empty());
        assert!(!Signal::new("abc").is_empty());
    }

    #[test]
    fn test_nth() {
        let signal = Signal::new("abcde");

        assert_eq!(signal.nth(0).unwrap(), 'a');
        assert_eq!(signal.nth(1).unwrap(), 'b');
        assert_eq!(signal.nth(2).unwrap(), 'c');
        assert_eq!(signal.nth(3).unwrap(), 'd');
        assert_eq!(signal.nth(4).unwrap(), 'e');
        assert_eq!(signal.nth(5), None);
        assert_eq!(signal.nth(42), None);
    }
}
