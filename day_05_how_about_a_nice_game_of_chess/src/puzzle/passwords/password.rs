pub const PASSWORD_LENGTH: usize = 8;

/// Trait supporting different passwords logic
pub trait Password {
    /// Create new instance
    fn new() -> Self;

    /// Return password
    fn get(&self) -> String;

    /// Update current password with provided MD5 sum
    fn update(&mut self, md5: &str);

    /// Check if password is complete
    fn is_complete(&self) -> bool;
}
