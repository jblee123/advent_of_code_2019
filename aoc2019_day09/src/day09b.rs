pub mod day09_cpu;

use aoc2019_utils;

use day09_cpu::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day09.txt");
    let prog = parse_prog(&input);
    let mut cpu = Cpu::new(&prog);
    cpu.add_input(2);
    cpu.exec_prog();
}
