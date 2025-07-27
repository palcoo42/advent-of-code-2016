#[derive(Debug, PartialEq)]
pub struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }

    pub fn is_valid(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(Triangle::new(3, 4, 5).is_valid());
        assert!(Triangle::new(10, 15, 20).is_valid());
        assert!(Triangle::new(10, 15, 24).is_valid());
    }

    #[test]
    fn test_is_valid_not() {
        assert!(!Triangle::new(5, 10, 25).is_valid());
        assert!(!Triangle::new(5, 10, 15).is_valid());
        assert!(!Triangle::new(42, 1, 2).is_valid());
    }
}
