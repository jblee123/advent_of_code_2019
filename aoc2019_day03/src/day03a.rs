pub mod day03_utils;

use aoc2019_utils;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day03.txt");
    let (wire1, wire2) = {
        let mut lines = input.lines();
        let line1 = lines.next().unwrap();
        let line2 = lines.next().unwrap();
        (line1, line2)
    };
    let moves1 = day03_utils::parse_moves(wire1);
    let moves2 = day03_utils::parse_moves(wire2);
    let result = day03_utils::get_closest_intersection_dist_from_moves(
        day03_utils::GridPoint{ x: 0, y: 0 },
        &moves1,
        &moves2,
    );
    match result {
        Some(dist) => println!("dist: {}", dist),
        None => println!("no intersection"),
    }
}
