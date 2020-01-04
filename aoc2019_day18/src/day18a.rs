pub mod day18_utils;

use std::time::Instant;

use aoc2019_utils::*;
use day18_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day18.txt");
    let (vault, pos, keys) = parse_input(&input);
    let start_time = Instant::now();
    let result = search_for_keys(pos, &vault, keys);
    println!(
        "search time: {}",
        sec_to_hrs_mins_secs_str(start_time.elapsed().as_secs()),
    );
    match result {
        Some(num_steps) => println!("min number of steps: {}", num_steps),
        None => println!("couldn't find a result"),
    }
}
