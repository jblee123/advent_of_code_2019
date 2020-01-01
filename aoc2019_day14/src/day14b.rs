pub mod day14_utils;

use day14_utils::*;

fn main() {
    const COLLECTED_ORE: usize = 1000000000000;

    let input = aoc2019_utils::get_input("inputs/day14.txt");
    let reactions = parse_input(&input);
    let depths = build_depths(&reactions);
    let result = get_max_fuel_for_ore(COLLECTED_ORE, &reactions, &depths);
    println!("max fuel: {}", result);
}
