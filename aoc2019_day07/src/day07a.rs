pub mod day07_cpu;
pub mod day07_utils;

use aoc2019_utils;

use day07_cpu::*;
use day07_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day07.txt");
    let prog = parse_prog(&input);
    let max_signal = find_max_of_all_phase_combos(&prog, &vec![0, 1, 2, 3, 4]);
    println!("max signal: {}", max_signal);
}
