pub mod day16_utils;

use day16_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day16.txt");
    let mut num_list = parse_input(&input);

    const PHASE_COUNT: usize = 100;
    for _ in 0..PHASE_COUNT {
        num_list = calc_next_phase_v3(&num_list);
    }

    let out_txt = get_output_text(&num_list, 0, 8);
    println!("result: {}", out_txt);
}
