pub mod day12_utils;

use day12_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day12.txt");
    let moon_positions = parse_input(&input);
    let mut moons = create_moons(&moon_positions);
    (0..1000).for_each(|_| { sim_time_step(&mut moons); });
    let system_energy = get_system_energy(&moons);
    println!("system energy: {}", system_energy);
}
