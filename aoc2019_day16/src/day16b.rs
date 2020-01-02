pub mod day16_utils;

use aoc2019_utils::*;
use day16_utils::*;

fn main() {
    use std::time::Instant;

    const INPUT_REPEAT_COUNT: usize = 10000;
    let input = aoc2019_utils::get_input("inputs/day16.txt");
    let num_list = parse_input(&input);
    let mut num_list = repeat_input(&num_list, INPUT_REPEAT_COUNT);
    println!("generated input of len: {}", num_list.len());

    let msg_offset = num_list[0..7].iter()
        .fold(0, |acc, n| acc * 10 + n) as usize;
    println!("message offset: {}", msg_offset);

    let start_time = Instant::now();
    let mut chunk_start_time = start_time;

    const PHASE_COUNT: usize = 100;
    for i in 0..PHASE_COUNT {
        num_list = calc_next_phase_v3(&num_list);

        println!("generated phase {}. phase time: {:.2}s, total time: {}",
            i,
            chunk_start_time.elapsed().as_secs_f32(),
            sec_to_hrs_mins_secs_str(start_time.elapsed().as_secs()));
        chunk_start_time = Instant::now();
    }

    let out_txt = get_output_text(&num_list, msg_offset, 8);
    println!("message: {}", out_txt);
}
