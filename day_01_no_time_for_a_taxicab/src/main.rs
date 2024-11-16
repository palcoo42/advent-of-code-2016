use std::process::exit;

use common::{env::project::Project, reader::text_reader::TextReader};
use puzzle::{city::City, puzzle_parser::PuzzleParser};

pub mod puzzle;

fn main() {
    println!("--- Day 1: No Time for a Taxicab ---");
    println!();

    let input_file = Project::new().resource_file("input.txt");
    let puzzle_parser = PuzzleParser::new();
    let document = TextReader::new(puzzle_parser)
        .read_with_hint(&input_file, 1)
        .unwrap_or_else(|err| {
            eprintln!(
                "Failed to parse file '{}' with error '{}'",
                input_file.to_str().unwrap(),
                err
            );
            exit(1);
        });

    let mut city = City::new();
    let blocks_to_hq = city.count_blocks_to_hq(document);

    println!("Part 1: Blocks to HQ: {}", blocks_to_hq);
}
