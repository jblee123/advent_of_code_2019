use std::collections::HashMap;
use std::str::FromStr;

use aoc2019_utils::*;

pub type GridPoint = point_2d::Point2d<i32>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn is_parallel_to(self, other: Dir) -> bool {
        let dir1_vert = match self {
            Dir::Up | Dir::Down => true,
            Dir::Left | Dir::Right => false,
        };
        let dir2_vert = match other {
            Dir::Up | Dir::Down => true,
            Dir::Left | Dir::Right => false,
        };

        dir1_vert == dir2_vert
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move {
    dir: Dir,
    dist: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Wire {
    dir: Dir,
    dist: u32,
}

const NUM_WIRES: usize = 2;
pub type GridSpace = [Option<Wire>; NUM_WIRES];

pub type WireGrid = HashMap<GridPoint, GridSpace>;

pub fn parse_moves(line: &str) -> Vec<Move> {
    line.split(',').map(|dir_str| {
        let first_byte = dir_str.as_bytes()[0];
        let dist = u32::from_str(&dir_str[1..]).unwrap();
        match first_byte {
            b'U' => Move { dir: Dir::Up, dist: dist },
            b'D' => Move { dir: Dir::Down, dist: dist },
            b'L' => Move { dir: Dir::Left, dist: dist },
            b'R' => Move { dir: Dir::Right, dist: dist },
            _ => panic!("bad dir indicator: {}", first_byte),
        }
    }).collect()
}

fn apply_moves_to_grid(
    start_pos: GridPoint,
    wire_num: usize,
    moves: &Vec<Move>,
    grid: &mut WireGrid,
) {
    let mut pos = start_pos;
    let mut wire_dist = 1u32;

    let mut moves = {
        let mut new_moves = moves.clone();
        new_moves.reverse();
        new_moves
    };

    while !moves.is_empty() {
        let mov = moves.last_mut().unwrap();
        let new_pos = match mov.dir {
            Dir::Up => GridPoint { x: pos.x, y: pos.y - 1 },
            Dir::Down => GridPoint { x: pos.x, y: pos.y + 1 },
            Dir::Left => GridPoint { x: pos.x - 1, y: pos.y },
            Dir::Right => GridPoint { x: pos.x + 1, y: pos.y },
        };

        let new_move = Move { dir: mov.dir, dist: mov.dist - 1 };

        let space = grid.entry(new_pos).or_insert([None; NUM_WIRES]);
        if !space[wire_num].is_some() {
            space[wire_num] = Some(Wire { dir: mov.dir, dist: wire_dist });
        }

        pos = new_pos;
        *mov = new_move;

        if new_move.dist == 0 {
            moves.pop();
        }

        wire_dist += 1;
    }
}

pub fn build_grid(
    start_pos: GridPoint,
    moves1: &Vec<Move>,
    moves2: &Vec<Move>,
) -> WireGrid {
    let mut grid = WireGrid::new();
    apply_moves_to_grid(start_pos, 0, &moves1, &mut grid);
    apply_moves_to_grid(start_pos, 1, &moves2, &mut grid);
    grid
}

pub fn get_closest_intersection(
    start_pos: GridPoint,
    wire1: &Vec<Move>,
    wire2: &Vec<Move>,
) -> Option<u32> {
    let grid = build_grid(start_pos, &wire1, &wire2);

    grid.iter()
        .filter(|(_, space)| {
            if let (Some(wire1), Some(wire2)) = (space[0], space[1]) {
                !wire1.dir.is_parallel_to(wire2.dir)
            } else {
                false
            }
        })
        .map(|(pos, _)| {
            let dx = (pos.x - start_pos.x).abs();
            let dy = (pos.y - start_pos.y).abs();
            (dx + dy) as u32
        })
        .min()
}

pub fn get_earliest_intersection(
    start_pos: GridPoint,
    wire1: &Vec<Move>,
    wire2: &Vec<Move>,
) -> Option<u32> {
    let grid = build_grid(start_pos, &wire1, &wire2);

    grid.iter()
        .filter(|(_, space)| {
            if let (Some(wire1), Some(wire2)) = (space[0], space[1]) {
                !wire1.dir.is_parallel_to(wire2.dir)
            } else {
                false
            }
        })
        .map(|(_, space)| {
            space[0].unwrap().dist + space[1].unwrap().dist
        })
        .min()
}

pub fn get_moves_from_input(file_name: &str) -> (Vec<Move>, Vec<Move>) {
    let input = aoc2019_utils::get_input(file_name);
    let (wire1, wire2) = {
        let mut lines = input.lines();
        let line1 = lines.next().unwrap();
        let line2 = lines.next().unwrap();
        (line1, line2)
    };
    let moves1 = parse_moves(wire1);
    let moves2 = parse_moves(wire2);

    (moves1, moves2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dir_is_parallel_to() {
        use super::*;

        assert_eq!(Dir::Up.is_parallel_to(Dir::Up), true);
        assert_eq!(Dir::Up.is_parallel_to(Dir::Down), true);
        assert_eq!(Dir::Down.is_parallel_to(Dir::Up), true);
        assert_eq!(Dir::Down.is_parallel_to(Dir::Down), true);
        assert_eq!(Dir::Left.is_parallel_to(Dir::Left), true);
        assert_eq!(Dir::Left.is_parallel_to(Dir::Right), true);
        assert_eq!(Dir::Right.is_parallel_to(Dir::Left), true);
        assert_eq!(Dir::Right.is_parallel_to(Dir::Right), true);

        assert_eq!(Dir::Up.is_parallel_to(Dir::Left), false);
        assert_eq!(Dir::Up.is_parallel_to(Dir::Right), false);
        assert_eq!(Dir::Down.is_parallel_to(Dir::Left), false);
        assert_eq!(Dir::Down.is_parallel_to(Dir::Right), false);
        assert_eq!(Dir::Left.is_parallel_to(Dir::Up), false);
        assert_eq!(Dir::Left.is_parallel_to(Dir::Down), false);
        assert_eq!(Dir::Right.is_parallel_to(Dir::Up), false);
        assert_eq!(Dir::Right.is_parallel_to(Dir::Down), false);
    }

    #[test]
    fn test_parse_dirs() {
        use super::*;

        let moves = parse_moves("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        assert_eq!(moves, vec![
            Move { dir: Dir::Right, dist: 75 },
            Move { dir: Dir::Down, dist: 30 },
            Move { dir: Dir::Right, dist: 83 },
            Move { dir: Dir::Up, dist: 83 },
            Move { dir: Dir::Left, dist: 12 },
            Move { dir: Dir::Down, dist: 49 },
            Move { dir: Dir::Right, dist: 71 },
            Move { dir: Dir::Up, dist: 7 },
            Move { dir: Dir::Left, dist: 72 },
        ]);
    }

    #[test]
    fn test_apply_moves_to_grid() {
        use super::*;

        let moves1 = parse_moves("R1,U2,L3,D4,L1,U1,R2");
        let moves2 = parse_moves("D1,L4");
        let start_pos = GridPoint { x: 0, y: 0 };

        let mut grid = WireGrid::new();

        apply_moves_to_grid(start_pos, 0, &moves1, &mut grid);

        let space_n1_1 = grid.get(&GridPoint { x: -1, y: 1});
        assert_eq!(
            space_n1_1,
            Some(&[
                Some(Wire { dir: Dir::Right, dist: 14 }),
                None,
            ]));
        let space_n2_1 = grid.get(&GridPoint { x: -2, y: 1});
        assert_eq!(
            space_n2_1,
            Some(&[
                Some(Wire { dir: Dir::Down, dist: 9 }),
                None,
            ]));

        apply_moves_to_grid(start_pos, 1, &moves2, &mut grid);

        let space_n2_1 = grid.get(&GridPoint { x: -2, y: 1});
        assert_eq!(
            space_n2_1,
            Some(&[
                Some(Wire { dir: Dir::Down, dist: 9 }),
                Some(Wire { dir: Dir::Left, dist: 3 }),
            ]));
    }

    #[test]
    fn test_get_closest_intersection() {
        use super::*;

        let moves1 = parse_moves("R8,U5,L5,D3");
        let moves2 = parse_moves("U7,R6,D4,L4");
        let result = get_closest_intersection(
            GridPoint { x: 0, y: 0 },
            &moves1,
            &moves2,
        );
        assert_eq!(result, Some(6));

        let moves1 = parse_moves("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let moves2 = parse_moves("U62,R66,U55,R34,D71,R55,D58,R83");
        let result = get_closest_intersection(
            GridPoint { x: 0, y: 0 },
            &moves1,
            &moves2,
        );
        assert_eq!(result, Some(159));

        let moves1 = parse_moves("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let moves2 = parse_moves("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let result = get_closest_intersection(
            GridPoint { x: 0, y: 0 },
            &moves1,
            &moves2,
        );
        assert_eq!(result, Some(135));
    }

    #[test]
    fn test_get_earliest_intersection() {
        use super::*;

        let moves1 = parse_moves("R8,U5,L5,D3");
        let moves2 = parse_moves("U7,R6,D4,L4");
        let result = get_earliest_intersection(
            GridPoint { x: 0, y: 0 },
            &moves1,
            &moves2,
        );
        assert_eq!(result, Some(30));

        let moves1 = parse_moves("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let moves2 = parse_moves("U62,R66,U55,R34,D71,R55,D58,R83");
        let result = get_earliest_intersection(
            GridPoint { x: 0, y: 0 },
            &moves1,
            &moves2,
        );
        assert_eq!(result, Some(610));

        let moves1 = parse_moves("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let moves2 = parse_moves("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let result = get_earliest_intersection(
            GridPoint { x: 0, y: 0 },
            &moves1,
            &moves2,
        );
        assert_eq!(result, Some(410));
    }
}
