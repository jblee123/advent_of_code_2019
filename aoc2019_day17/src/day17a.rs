pub mod day17_cpu;
pub mod day17_utils;

use day17_cpu::*;
use day17_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day17.txt");
    let prog = parse_prog(&input);

    let mut cpu = Cpu::new(&prog);
    cpu.set_print_output(false);
    cpu.exec_prog();
    let scaf_map = cpu.get_output();

    let (scaf_map, robot_pose) = parse_map_from_robot(&scaf_map);
    print_map(&scaf_map, &robot_pose);

    let alignment_param = get_alignment_param(&scaf_map);
    println!("alignment param: {}", alignment_param);
}
