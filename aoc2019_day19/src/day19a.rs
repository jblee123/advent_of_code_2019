pub mod day19_cpu;

use day19_cpu::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day19.txt");
    let prog = parse_prog(&input);

    let mut num_affected_points = 0;

    println!("the grid:");
    const GRID_WIDTH: i64 = 50;
    const GRID_HEIGHT: i64 = 50;
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let mut cpu = Cpu::new(&prog);
            cpu.set_print_output(false);
            cpu.add_input(x);
            cpu.add_input(y);
            cpu.exec_prog();
            let is_affected = cpu.pop_output().unwrap() != 0;
            let out_char = if is_affected { '#' } else { '.' };
            num_affected_points += if is_affected { 1 } else { 0 };
            print!("{}", out_char);
        }
        println!("");
    }

    println!("affected points: {}", num_affected_points);
}
