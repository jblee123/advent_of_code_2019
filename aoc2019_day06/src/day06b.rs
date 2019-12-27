pub mod day06_utils;

use aoc2019_utils;

use day06_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day06.txt");
    let orbit_sys = parse_input(&input);
    let num_transfers = get_num_transfers(&orbit_sys, "YOU", "SAN");
    println!("num transfers: {}", num_transfers);
}
