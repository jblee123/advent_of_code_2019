use std::str::FromStr;

use aoc2019_utils::*;

pub type GridPoint = Point2d<i32>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn apply_move(pos: GridPoint, movement: Move) -> GridPoint {
    match movement {
        Move::Up(dist) => GridPoint{ x: pos.x, y: pos.y - dist },
        Move::Down(dist) => GridPoint{ x: pos.x, y: pos.y + dist },
        Move::Left(dist) => GridPoint{ x: pos.x - dist, y: pos.y },
        Move::Right(dist) => GridPoint{ x: pos.x + dist, y: pos.y },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_dirs() {
        use super::*;

        let moves = parse_moves("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        assert_eq!(moves, vec![
            Move::Right(75),
            Move::Down(30),
            Move::Right(83),
            Move::Up(83),
            Move::Left(12),
            Move::Down(49),
            Move::Right(71),
            Move::Up(7),
            Move::Left(72),
        ]);
    }

    #[test]
    fn test_apply_move() {
        use super::*;

        let pos = GridPoint{ x: 5, y: 6 };

        let result = apply_move(pos, Move::Left(5));
        assert_eq!(result, GridPoint{ x: 0, y: 6 });

        let result = apply_move(pos, Move::Right(5));
        assert_eq!(result, GridPoint{ x: 10, y: 6 });

        let result = apply_move(pos, Move::Up(5));
        assert_eq!(result, GridPoint{ x: 5, y: 1 });

        let result = apply_move(pos, Move::Down(5));
        assert_eq!(result, GridPoint{ x: 5, y: 11 });
    }

    #[test]
    fn test_moves_to_points() {
        use super::*;

        let result = moves_to_points(
            GridPoint{ x: 0, y: 0 },
            &vec![
                Move::Right(1),
                Move::Up(2),
                Move::Left(3),
                Move::Down(4),
            ]);
        assert_eq!(result, vec![
            GridPoint{ x: 0, y: 0 },
            GridPoint{ x: 1, y: 0 },
            GridPoint{ x: 1, y: -2 },
            GridPoint{ x: -2, y: -2 },
            GridPoint{ x: -2, y: 2 },
        ]);
    }

    #[test]
    fn test_get_intersection() {
        use super::*;

        let result = get_intersection(
            GridPoint{ x: 2, y: 5 }, GridPoint{ x: 10, y: 5 },
            GridPoint{ x: 7, y: 3 }, GridPoint{ x: 7, y: 11 });
        assert_eq!(result, Some(GridPoint{ x: 7, y: 5 }));

        let result = get_intersection(
            GridPoint{ x: 10, y: 5 }, GridPoint{ x: 2, y: 5 },
            GridPoint{ x: 7, y: 3 }, GridPoint{ x: 7, y: 11 });
        assert_eq!(result, Some(GridPoint{ x: 7, y: 5 }));

        let result = get_intersection(
            GridPoint{ x: 2, y: 5 }, GridPoint{ x: 10, y: 5 },
            GridPoint{ x: 7, y: 11 }, GridPoint{ x: 7, y: 3 });
        assert_eq!(result, Some(GridPoint{ x: 7, y: 5 }));

        let result = get_intersection(
            GridPoint{ x: 2, y: 5 }, GridPoint{ x: 2, y: 6 },
            GridPoint{ x: 2, y: 5 }, GridPoint{ x: 7, y: 5 });
        assert_eq!(result, Some(GridPoint{ x: 2, y: 5 }));

        let result = get_intersection(
            GridPoint{ x: 2, y: 5 }, GridPoint{ x: 2, y: 6 },
            GridPoint{ x: 9, y: 6 }, GridPoint{ x: 2, y: 6 });
        assert_eq!(result, Some(GridPoint{ x: 2, y: 6 }));

        let result = get_intersection(
            GridPoint{ x: 2, y: 5 }, GridPoint{ x: 10, y: 5 },
            GridPoint{ x: 7, y: 0 }, GridPoint{ x: 7, y: 1 });
        assert_eq!(result, None);

        let result = get_intersection(
            GridPoint{ x: 2, y: 5 }, GridPoint{ x: 10, y: 5 },
            GridPoint{ x: 7, y: 11 }, GridPoint{ x: 7, y: 10 });
        assert_eq!(result, None);

        let result = get_intersection(
            GridPoint{ x: 2, y: 5 }, GridPoint{ x: 3, y: 5 },
            GridPoint{ x: 7, y: 11 }, GridPoint{ x: 7, y: 3 });
        assert_eq!(result, None);

        let result = get_intersection(
            GridPoint{ x: 8, y: 5 }, GridPoint{ x: 10, y: 5 },
            GridPoint{ x: 7, y: 11 }, GridPoint{ x: 7, y: 3 });
        assert_eq!(result, None);
    }

    #[test]
    fn get_closest_intersection_dist_from_moves() {
        use super::*;

        let moves1 = parse_moves("R8,U5,L5,D3");
        let moves2 = parse_moves("U7,R6,D4,L4");
        let result = get_closest_intersection_dist_from_moves(
            GridPoint{ x: 0, y: 0 },
            &moves1,
            &moves2,
        );
        assert_eq!(result, Some(6));
    }
}

pub fn parse_moves(line: &str) -> Vec<Move> {
    line.split(',').map(|dir_str| {
        let first_byte = dir_str.as_bytes()[0];
        match first_byte {
            b'U' => Move::Up(i32::from_str(&dir_str[1..]).unwrap()),
            b'D' => Move::Down(i32::from_str(&dir_str[1..]).unwrap()),
            b'L' => Move::Left(i32::from_str(&dir_str[1..]).unwrap()),
            b'R' => Move::Right(i32::from_str(&dir_str[1..]).unwrap()),
            _ => panic!("bad dir indicator: {}", first_byte),
        }
    }).collect()
}

fn moves_to_points(start_pos: GridPoint, dirs: &Vec<Move>) -> Vec<GridPoint> {
    let mut points = vec![start_pos];

    for dir in dirs {
        points.push(apply_move(*points.last().unwrap(), *dir));
    }

    points
}

fn get_intersection(
    p1_a: GridPoint,
    p1_b: GridPoint,
    p2_a: GridPoint,
    p2_b: GridPoint,
) -> Option<GridPoint> {
    assert!(p1_a.x == p1_b.x || p1_a.y == p1_b.y);
    assert!(p2_a.x == p2_b.x || p2_a.y == p2_b.y);

    let p1_horiz = p1_a.y == p1_b.y;
    let p2_horiz = p2_a.y == p2_b.y;

    if p1_horiz == p2_horiz {
        return None;
    }

    if p1_horiz {
        let min_p1_x = std::cmp::min(p1_a.x, p1_b.x);
        let max_p1_x = std::cmp::max(p1_a.x, p1_b.x);
        let min_p2_y = std::cmp::min(p2_a.y, p2_b.y);
        let max_p2_y = std::cmp::max(p2_a.y, p2_b.y);
        let intersects = min_p1_x <= p2_a.x && p2_a.x <= max_p1_x
            && min_p2_y <= p1_a.y && p1_a.y <= max_p2_y;
        if intersects {
            Some(GridPoint{ x: p2_a.x, y: p1_a.y })
        } else {
            None
        }
    } else {
        let min_p1_y = std::cmp::min(p1_a.y, p1_b.y);
        let max_p1_y = std::cmp::max(p1_a.y, p1_b.y);
        let min_p2_x = std::cmp::min(p2_a.x, p2_b.x);
        let max_p2_x = std::cmp::max(p2_a.x, p2_b.x);
        let intersects = min_p1_y <= p2_a.y && p2_a.y <= max_p1_y
            && min_p2_x <= p1_a.x && p1_a.x <= max_p2_x;
        if intersects {
            Some(GridPoint{ x: p1_a.x, y: p2_a.y })
        } else {
            None
        }
    }
}

fn get_closest_intersection_dist_from_points(
    start_pos: GridPoint,
    wire1: &Vec<GridPoint>,
    wire2: &Vec<GridPoint>,
) -> Option<u32> {
    let num_wire1_segs = wire1.len() - 1;
    let num_wire2_segs = wire2.len() - 1;
    let mut min_dist = std::u32::MAX;
    for i in 0..num_wire1_segs {
        for j in 0..num_wire2_segs {
            if i == 0 && j == 0 {
                continue;
            }

            let p1_a = &wire1[i];
            let p1_b = &wire1[i + 1];
            let p2_a = &wire2[j];
            let p2_b = &wire2[j + 1];

            if let Some(pnt) = get_intersection(*p1_a, *p1_b, *p2_a, *p2_b) {
                let dist = (start_pos.x - pnt.x).abs()
                    + (start_pos.y - pnt.y).abs();
                min_dist = std::cmp::min(min_dist, dist as u32);
            }
        }
    }

    if min_dist < std::u32::MAX {
        Some(min_dist)
    } else {
        None
    }
}

pub fn get_closest_intersection_dist_from_moves(
    start_pos: GridPoint,
    wire1: &Vec<Move>,
    wire2: &Vec<Move>,
) -> Option<u32> {
    let wire1 = moves_to_points(start_pos, wire1);
    let wire2 = moves_to_points(start_pos, wire2);
    get_closest_intersection_dist_from_points(start_pos, &wire1, &wire2)
}
