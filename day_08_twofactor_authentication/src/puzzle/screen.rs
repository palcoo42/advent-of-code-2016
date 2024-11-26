use super::{light::Light, position::Position};

pub struct Screen {
    rows: usize,
    cols: usize,
    lights: Vec<Vec<Light>>,
}

impl Screen {
    pub fn new(rows: usize, cols: usize) -> Self {
        let lights = (0..rows)
            .map(|_| (0..cols).map(|_| Light::Off).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { rows, cols, lights }
    }

    pub fn set_light(&mut self, position: &Position, light: Light) {
        // Check position validity
        if !self.is_position_valid(position) {
            panic!(
                "Invalid position '{:?}', Screen rows: {}, cols: {}",
                position, self.rows, self.cols
            );
        }

        self.lights[position.x][position.y] = light;
    }

    fn is_position_valid(&self, position: &Position) -> bool {
        position.x < self.rows && position.y < self.cols
    }

    pub fn get_column(&self, col: usize) -> Vec<Light> {
        if col >= self.cols {
            panic!("Failed to fetch column '{}' [out of bounds]", col);
        }

        (0..self.rows)
            .map(|r| self.lights[r][col].clone())
            .collect::<Vec<_>>()
    }

    pub fn get_row(&self, row: usize) -> Vec<Light> {
        if row >= self.rows {
            panic!("Failed to fetch row '{}' [out of bounds]", row);
        }

        (0..self.cols)
            .map(|c| self.lights[row][c].clone())
            .collect::<Vec<_>>()
    }

    pub fn count_lit_pixels(&self) -> usize {
        self.lights
            .iter()
            .flatten()
            .filter(|&light| *light == Light::On)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::commands::{
        rectangle::Rectangle, rotate_column::RotateColumn, rotate_row::RotateRow,
    };

    use super::*;

    pub fn create_screen() -> Screen {
        Screen::new(3, 7)
    }

    pub fn assert_lights(screen: &Screen, on: &[Position]) {
        for row in 0..screen.rows {
            for col in 0..screen.cols {
                // If position is inside 'on' we check for Light::On
                let light = match on.iter().find(|&pos| row == pos.x && col == pos.y) {
                    Some(_) => Light::On,
                    None => Light::Off,
                };

                assert_eq!(
                    screen.lights[row][col], light,
                    "screen: {:?}",
                    screen.lights
                );
            }
        }
    }

    #[test]
    fn test_is_position_valid() {
        let screen = create_screen();

        assert!(screen.is_position_valid(&Position { x: 0, y: 0 }));
        assert!(screen.is_position_valid(&Position { x: 0, y: 6 }));
        assert!(!screen.is_position_valid(&Position { x: 0, y: 7 }));

        assert!(screen.is_position_valid(&Position { x: 1, y: 0 }));
        assert!(screen.is_position_valid(&Position { x: 1, y: 6 }));
        assert!(!screen.is_position_valid(&Position { x: 1, y: 7 }));

        assert!(screen.is_position_valid(&Position { x: 2, y: 0 }));
        assert!(screen.is_position_valid(&Position { x: 2, y: 6 }));
        assert!(!screen.is_position_valid(&Position { x: 2, y: 7 }));

        assert!(!screen.is_position_valid(&Position { x: 3, y: 0 }));
        assert!(!screen.is_position_valid(&Position { x: 3, y: 6 }));
        assert!(!screen.is_position_valid(&Position { x: 3, y: 7 }));
    }

    #[test]
    fn test_screen() {
        let mut screen = create_screen();
        assert_lights(&screen, &[]);

        // Run command 'Rectangle'
        let cmd = Rectangle::new(3, 2);
        cmd.draw_on_screen(&mut screen);

        let on = vec![
            Position { x: 0, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 0, y: 2 },
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
        ];
        assert_lights(&screen, &on);

        // Run command 'RotateColumn'
        let cmd = RotateColumn::new(1, 1);
        cmd.draw_on_screen(&mut screen);

        let on = vec![
            Position { x: 0, y: 0 },
            Position { x: 0, y: 2 },
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 1 },
        ];
        assert_lights(&screen, &on);

        // Run command 'RotateRow'
        let cmd = RotateRow::new(0, 4);
        cmd.draw_on_screen(&mut screen);

        let on = vec![
            Position { x: 0, y: 4 },
            Position { x: 0, y: 6 },
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 1 },
        ];
        assert_lights(&screen, &on);

        // Run command 'RotateColumn'
        let cmd = RotateColumn::new(1, 1);
        cmd.draw_on_screen(&mut screen);

        let on = vec![
            Position { x: 0, y: 1 },
            Position { x: 0, y: 4 },
            Position { x: 0, y: 6 },
            Position { x: 1, y: 0 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 1 },
        ];
        assert_lights(&screen, &on);
    }
}
