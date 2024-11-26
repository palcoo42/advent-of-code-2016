use crate::puzzle::{light::Light, screen::Screen};
use std::{any::Any, fmt::Debug};

pub trait Command: Debug + Any {
    /// Draw command on the screen
    ///
    /// # Arguments
    ///
    /// _screen_ - Screen where command will be draw
    fn draw_on_screen(&self, screen: &mut Screen);

    /// Rotate light by the shift to the right
    fn rotate(&self, lights: &mut Vec<Light>, shift: usize) {
        let length = lights.len();

        // Make enough space for shifted items
        let mut rotated = vec![Light::Off; length + shift];

        rotated[0..shift].clone_from_slice(&lights[length - shift..length]);
        rotated[shift..length + shift].clone_from_slice(&lights[0..length]);

        lights[0..length].clone_from_slice(&rotated[0..length]);
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::commands::rotate_column::RotateColumn;

    use super::*;

    #[test]
    fn test_rotate() {
        let cmd = RotateColumn::new(42, 42);
        let mut lights = vec![Light::Off, Light::Off, Light::On, Light::Off];

        cmd.rotate(&mut lights, 1);
        assert_eq!(lights, vec![Light::Off, Light::Off, Light::Off, Light::On]);

        cmd.rotate(&mut lights, 1);
        assert_eq!(lights, vec![Light::On, Light::Off, Light::Off, Light::Off]);

        cmd.rotate(&mut lights, 2);
        assert_eq!(lights, vec![Light::Off, Light::Off, Light::On, Light::Off]);

        cmd.rotate(&mut lights, 3);
        assert_eq!(lights, vec![Light::Off, Light::On, Light::Off, Light::Off]);

        cmd.rotate(&mut lights, 4);
        assert_eq!(lights, vec![Light::Off, Light::On, Light::Off, Light::Off]);
    }
}
