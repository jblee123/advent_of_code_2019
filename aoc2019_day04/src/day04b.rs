pub mod day04_utils;

use aoc2019_utils;

use day04_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day04.txt");
    let (start, end) = extract_range(&input);
    let num_passwords = count_passwords_in_range_v2(start, end);
    println!("num passwords: {}", num_passwords);
}
