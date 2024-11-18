use super::puzzle::{PuzzleResult, SolutionResult};

/// Defines requirements for real puzzle solver which can be used to solve Advent of Code puzzles.
pub trait PuzzleSolver {
    /// Creates a new instance of puzzle solver
    ///
    /// # Returns
    ///
    /// New instance of puzzle solver
    fn new() -> Self;

    /// Parse content of the input file
    /// This method is called only if [Puzzle] is created with reader
    ///
    /// # Arguments
    ///
    /// _lines_ - Lines read from input file
    ///
    /// # Returns
    ///
    /// Empty result on success, error on failure
    fn parse_input_file(&mut self, lines: Vec<String>) -> PuzzleResult;

    /// Solve part 1 of the puzzle
    ///
    /// # Returns
    ///
    /// String representation of the solution on success, error on failure
    fn part_1(&self) -> SolutionResult;

    /// Solve part 2 of the puzzle
    ///
    /// # Returns
    ///
    /// String representation of the solution on success, error on failure
    fn part_2(&self) -> SolutionResult;
}