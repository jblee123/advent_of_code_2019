pub mod day06_utils;

use aoc2019_utils;

use day06_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day06.txt");
    let orbit_sys = parse_input(&input);
    let num_orbits = get_total_orbits(&orbit_sys);
    println!("total orbits: {}", num_orbits);
}
