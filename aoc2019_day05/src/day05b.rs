pub mod day05_utils;

use aoc2019_utils;

use day05_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day05.txt");
    let prog = parse_prog(&input);
    let mut cpu = Cpu::new(prog);
    cpu.add_input(5);
    cpu.exec_prog();
}
