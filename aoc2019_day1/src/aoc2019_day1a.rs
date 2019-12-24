pub mod aoc2019_day1_utils;

use aoc2019_utils;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day01.txt");
    let fuel = aoc2019_day1_utils::calc_fuel_from_str(&input);
    println!("total fuel: {}", fuel);
}
