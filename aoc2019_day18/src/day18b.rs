pub mod day18_utils;

use std::time::Instant;

use day18_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day18.txt");
    let (vault, pos) = parse_input(&input);
    let (vault, positions) = replace_vault_center(&vault, pos);

    let start_time = Instant::now();
    let result = search_for_keys(&vault, &positions);
    println!("search time: {}s", start_time.elapsed().as_secs_f32());
    match result {
        Some(num_steps) => println!("min number of steps: {}", num_steps),
        None => println!("couldn't find a result"),
    }
}
