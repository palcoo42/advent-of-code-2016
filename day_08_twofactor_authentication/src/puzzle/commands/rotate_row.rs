use crate::puzzle::{position::Position, screen::Screen};

use super::command::Command;

#[derive(Debug)]
pub struct RotateRow {
    row: usize,
    shift: usize,
}

impl RotateRow {
    pub fn new(row: usize, shift: usize) -> Self {
        Self { row, shift }
    }
}

impl Command for RotateRow {
    fn draw_on_screen(&self, screen: &mut Screen) {
        // Read column items
        let mut lights = screen.get_row(self.row);

        self.rotate(&mut lights, self.shift);

        // Update screen
        for (col, light) in lights.iter().enumerate() {
            screen.set_light(
                &Position {
                    x: self.row,
                    y: col,
                },
                light.clone(),
            );
        }
    }
}
