use advent_of_code::puzzles::puzzle::{Puzzle, PuzzleResult};
use puzzle::solver::Solver;

pub mod puzzle;

fn main() -> PuzzleResult {
    let mut puzzle = Puzzle::<Solver>::new();

    puzzle.solve()
}
