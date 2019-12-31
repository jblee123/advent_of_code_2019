pub mod day13_cpu;

use std::collections::HashMap;

use aoc2019_utils::*;

use day13_cpu::*;

type Coord = point_2d::Point2d<i16>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile { Empty, Wall, Block, Paddle, Ball }

impl Tile {
    fn from_num(num: i16) -> Self {
        match num {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("bad tile num"),
        }
    }
}

fn main() {
    let input = get_input("inputs/day13.txt");
    let prog = parse_prog(&input);
    let mut cpu = Cpu::new(&prog);
    cpu.set_print_output(false);

    let mut screen = HashMap::new();
    let mut to_draw = vec![];

    loop {
        let state = cpu.exec();

        if let Some(output) = cpu.pop_output() {
            to_draw.push(output as i16);
            if to_draw.len() == 3 {
                screen.insert(
                    Coord{ x: to_draw[0], y: to_draw[1]},
                    Tile::from_num(to_draw[2]),
                );
                to_draw.clear();
            }
        }

        if state == CpuState::Done {
            break;
        }
    }

    let block_count = screen.iter()
        .filter(|(_, tile)| **tile == Tile::Block)
        .count();

    println!("block count: {}", block_count);
}
