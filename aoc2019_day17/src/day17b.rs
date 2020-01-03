pub mod day17_cpu;
pub mod day17_utils;

use day17_cpu::*;
use day17_utils::*;

fn main() {
    // These were found by inspection.
    const FUNC_A: &str = "L,12,L,10,R,8,L,12\n";
    const FUNC_B: &str = "R,8,R,10,R,12\n";
    const FUNC_C: &str = "L,10,R,12,R,8\n";
    const FUNC_MAIN: &str = "A,B,A,B,C,C,B,A,B,C\n";
    const VIEW_FEED: &str = "n\n";

    let cmd_main = ascii_to_vec(FUNC_MAIN);
    let cmd_a = ascii_to_vec(FUNC_A);
    let cmd_b = ascii_to_vec(FUNC_B);
    let cmd_c = ascii_to_vec(FUNC_C);
    let cmd_view_feed = ascii_to_vec(VIEW_FEED);

    let input = aoc2019_utils::get_input("inputs/day17.txt");
    let prog = parse_prog(&input);

    let mut cpu = Cpu::new(&prog);
    cpu.set_mem_at(0, 2);
    cpu.set_print_output(false);
    cpu.add_input_from_slice(&cmd_main[..]);
    cpu.add_input_from_slice(&cmd_a[..]);
    cpu.add_input_from_slice(&cmd_b[..]);
    cpu.add_input_from_slice(&cmd_c[..]);
    cpu.add_input_from_slice(&cmd_view_feed[..]);

    cpu.exec_prog();

    let cpu_output = cpu.get_output();
    let dust_amount = cpu_output.last().unwrap();
    println!("dust amount: {}", dust_amount);
}
