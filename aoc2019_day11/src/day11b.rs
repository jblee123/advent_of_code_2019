pub mod day11_cpu;
pub mod day11_utils;

use day11_cpu::*;
use day11_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day11.txt");
    let prog = parse_prog(&input);
    let grid = run_robot_sim(&prog, true);
    println!("the robot painted {} tiles", grid.len());
    print_grid(&grid);
}
