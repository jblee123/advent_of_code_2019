pub mod day02_utils;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day02.txt");
    let prog = day02_utils::parse_prog(&input);
    let mut cpu = day02_utils::Cpu::new(prog);
    cpu.mem[1] = 12;
    cpu.mem[2] = 2;
    cpu.exec_prog();
    println!("mem[0]: {}", cpu.mem[0]);
}
