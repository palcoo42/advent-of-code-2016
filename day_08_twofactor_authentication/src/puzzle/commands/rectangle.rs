use crate::puzzle::{light::Light, position::Position, screen::Screen};

use super::command::Command;

#[derive(Debug)]
pub struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Command for Rectangle {
    fn draw_on_screen(&self, screen: &mut Screen) {
        // Light all pixel in the rectangle
        for row in 0..self.height {
            for col in 0..self.width {
                screen.set_light(&Position { x: row, y: col }, Light::On);
            }
        }
    }
}
