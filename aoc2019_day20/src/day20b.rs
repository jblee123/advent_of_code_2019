pub mod day20_utils;

use day20_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day20.txt");
    let maze = parse_input(&input);
    let result = traverse_recursive_maze(&maze);
    match result {
        Some(num_steps) => println!("path is {} steps", num_steps),
        None => println!("no path found"),
    }
}
