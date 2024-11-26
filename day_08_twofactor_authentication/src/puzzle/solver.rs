use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{commands::command::Command, parser::Parser, screen::Screen};

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

pub struct Solver {
    commands: Vec<Box<dyn Command>>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 08: Two-Factor Authentication ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.commands = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let mut screen = Screen::new(SCREEN_HEIGHT, SCREEN_WIDTH);

        for cmd in &self.commands {
            cmd.draw_on_screen(&mut screen);
        }

        Ok(screen.count_lit_pixels().to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let mut screen = Screen::new(SCREEN_HEIGHT, SCREEN_WIDTH);

        for cmd in &self.commands {
            cmd.draw_on_screen(&mut screen);
        }

        Ok(screen.get_code())
    }
}
