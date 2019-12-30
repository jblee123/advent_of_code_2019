pub mod day10_utils;

use aoc2019_utils;
use day10_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day10.txt");
    let grid = parse_input(&input);
    let loc = max_visible_asteroids_loc(&grid);
    match loc {
        Some((num_visible, coord)) => {
            println!("Best is [{}, {}] with {} visible.",
                coord.x, coord.y, num_visible);

            match get_nth_shot(&grid, coord, 200) {
                Some(shot_coord) => {
                    let val = shot_coord.x * 100 + shot_coord.y;
                    println!("200th shot at: {}, {}",
                        shot_coord.x, shot_coord.y);
                    println!("val: {}", val);
                },
                None => println!("Too few asteroids shot"),
            }
        },
        None => println!("No asteroids found."),
    }
}
