use crate::puzzle::{position::Position, screen::Screen};

use super::command::Command;

#[derive(Debug)]
pub struct RotateColumn {
    col: usize,
    shift: usize,
}

impl RotateColumn {
    pub fn new(col: usize, shift: usize) -> Self {
        Self { col, shift }
    }
}

impl Command for RotateColumn {
    fn draw_on_screen(&self, screen: &mut Screen) {
        // Read column items
        let mut lights = screen.get_column(self.col);

        self.rotate(&mut lights, self.shift);

        // Update screen
        for (row, light) in lights.iter().enumerate() {
            screen.set_light(
                &Position {
                    x: row,
                    y: self.col,
                },
                light.clone(),
            );
        }
    }
}
