pub mod day15_cpu;
pub mod day15_utils;

use day15_cpu::*;
use day15_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day15.txt");
    let prog = parse_prog(&input);
    println!("prog len: {}", prog.len());
    let (ship_map, start_pos) = create_map(&prog);
    let oxygen_pos = ship_map.get_oxygen_pos();
    println!("ship map bounds: top: {}, left: {}, bottom: {}, right: {}",
        ship_map.top, ship_map.left, ship_map.bottom, ship_map.right);
    println!("  with {} tiles", ship_map.tiles.len());
    print_map(&ship_map);
    println!("");
    if !oxygen_pos.is_some() {
        println!("no oxygen found");
        return;
    }

    let oxygen_pos = oxygen_pos.unwrap();
    println!("oxygen at: {}, {}", oxygen_pos.x, oxygen_pos.y);

    match find_shortest_path_len(&ship_map, start_pos, oxygen_pos) {
        None => println!("no path to oxygen"),
        Some(dist) => println!("shortest path: {}", dist),
    }
}
