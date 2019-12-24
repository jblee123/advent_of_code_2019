pub mod day01_utils;

use aoc2019_utils;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day01.txt");
    let fuel = day01_utils::calc_fuel_for_fuel_from_str(&input);
    println!("total fuel: {}", fuel);
}
