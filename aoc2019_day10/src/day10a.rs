pub mod day10_utils;

use aoc2019_utils;
use day10_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day10.txt");
    let grid = parse_input(&input);
    let result = max_visible_asteroids_loc(&grid);
    match result {
        Some((num_visible, coord)) => {
            println!("Best is [{}, {}] with {} visible.",
                coord.x, coord.y, num_visible);
        },
        None => {
            println!("No asteroids found.")
        }
    }
}
