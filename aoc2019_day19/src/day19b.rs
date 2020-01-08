pub mod day19_cpu;

use day19_cpu::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day19.txt");
    let prog = parse_prog(&input);

    let is_affected = |x, y| {
        let mut cpu = Cpu::new(&prog);
        cpu.set_print_output(false);
        cpu.add_input(x);
        cpu.add_input(y);
        cpu.exec_prog();
        cpu.pop_output().unwrap() != 0
    };

    let find_first_affected_x = |mut x, y| {
        while !is_affected(x, y) {
            x += 1;
        }
        x
    };

    const TARGET_SIZE: i64 = 100;
    const SIDE_OFFSET: i64 = TARGET_SIZE - 1;
    let mut x = 0;
    let mut y = SIDE_OFFSET;
    x = find_first_affected_x(x, y);

    while !is_affected(x + SIDE_OFFSET, y - SIDE_OFFSET) {
        y += 1;
        x = find_first_affected_x(x, y);
    }

    let target_x = x;
    let target_y = y - SIDE_OFFSET;
    let answer = target_x * 10000 + target_y;

    println!("answer = {}, {} => {}", target_x, target_y, answer);
}
