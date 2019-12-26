pub mod day03_utils;

use day03_utils::*;

fn main() {
    const START_POS: GridPoint = GridPoint { x: 0, y: 0 };

    let (moves1, moves2) = get_moves_from_input("inputs/day03.txt");
    let result = get_earliest_intersection(START_POS, &moves1, &moves2);
    match result {
        Some(steps) => println!("steps: {}", steps),
        None => println!("no intersection"),
    }
}
