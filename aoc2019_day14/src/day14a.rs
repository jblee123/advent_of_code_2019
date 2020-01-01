pub mod day14_utils;

use day14_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day14.txt");
    let reactions = parse_input(&input);
    let depths = build_depths(&reactions);
    let result = get_needed_ore_for_fuel(1, &reactions, &depths);
    println!("ore needed: {}", result);
}
