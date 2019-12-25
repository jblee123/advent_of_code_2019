pub mod day02_utils;

fn do_search(prog: &Vec<i64>) {
    const TARGET_NUM: i64 = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut cpu = day02_utils::Cpu::new(prog.clone());
            cpu.mem[1] = noun;
            cpu.mem[2] = verb;
            cpu.exec_prog();

            if cpu.mem[0] == TARGET_NUM {
                println!("noun/verb: {}/{}", noun, verb);
                println!("final answer: {}", noun * 100 + verb);
                return;
            }
        }
    }

    println!("no solution found");
}

fn main() {
    let input = aoc2019_utils::get_input("inputs/day02.txt");
    let prog = day02_utils::parse_prog(&input);
    do_search(&prog);
}
