pub mod day21_cpu;

use day21_cpu::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day21.txt");
    let prog = parse_prog(&input);

    // death = !D || (!E && !H) = !(D && !(!E && !H))
    // hole = !(A && B && C)
    // jump = hole && !death = !(A && B && C) && D && !(!E && !H)
    let script = concat!(
        // J = !(!E && !H)
        "NOT E T\n",
        "NOT H J\n",
        "AND T J\n",
        "NOT J J\n",

        // J &= D
        "AND D J\n",

        // T = !(A && B && C)
        "NOT A T\n",
        "NOT T T\n",
        "AND B T\n",
        "AND C T\n",
        "NOT T T\n",

        // J &= T
        "AND T J\n",

        "RUN\n",
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
