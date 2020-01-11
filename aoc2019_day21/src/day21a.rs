pub mod day21_cpu;

use day21_cpu::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day21.txt");
    let prog = parse_prog(&input);

    // !(A && B && C) && D
    let script = concat!(
        // J = !(A && B && C)
        "NOT T T\n",
        "AND A T\n",
        "AND B T\n",
        "AND C T\n",
        "NOT T J\n",

        // J &= D
        "NOT D T\n",
        "NOT T T\n",
        "AND T J\n",

        "WALK\n",
    );

    let script_input = script.bytes()
        .map(|c| c as i64)
        .collect::<Vec<i64>>();

    let mut cpu = Cpu::new(&prog);
    cpu.set_print_output(false);
    cpu.add_input_from_slice(&script_input);
    cpu.exec_prog();
    let output = cpu.get_output();

    if output.is_empty() {
        println!("NO OUTPUT!");
        return;
    }

    let last_val = *(output.last().unwrap());
    if last_val > 127 {
        println!("damage value: {}", last_val);
        return;
    }

    let out_bytes = output.iter().map(|o| *o as u8).collect::<Vec<u8>>();
    let out_str = std::str::from_utf8(&out_bytes).unwrap();
    println!("{}", out_str);
}
