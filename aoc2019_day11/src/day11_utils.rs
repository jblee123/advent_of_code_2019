use std::collections::HashMap;

use aoc2019_utils::*;
use crate::day11_cpu::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TurnDir { Left, Right }

impl TurnDir {
    fn from_num(num: i64) -> Self {
        match num {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("invalid TurnDir num: {}", num),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn turn(self, turn_dir: TurnDir) -> Self {
        match self {
            Self::Up => match turn_dir {
                TurnDir::Left => Self::Left,
                TurnDir::Right => Self::Right,
            },
            Self::Down => match turn_dir {
                TurnDir::Left => Self::Right,
                TurnDir::Right => Self::Left,
            },
            Self::Left => match turn_dir {
                TurnDir::Left => Self::Down,
                TurnDir::Right => Self::Up,
            },
            Self::Right => match turn_dir {
                TurnDir::Left => Self::Up,
                TurnDir::Right => Self::Down,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TileColor {
    Black,
    White,
}

impl TileColor {
    const BLACK: i64 = 0;
    const WHITE: i64 = 1;

    fn from_num(num: i64) -> Self {
        match num {
            Self::BLACK => Self::Black,
            Self::WHITE => Self::White,
            _ => panic!("invalid TileColor num: {}", num),
        }
    }

    fn to_num(self) -> i64 {
        match self {
            Self::Black => Self:: BLACK,
            Self::White => Self:: WHITE,
        }
    }
}

pub type Coord = point_2d::Point2d<i32>;

pub type Grid = HashMap<Coord, TileColor>;

pub struct Robot {
    pos: Coord,
    dir: Dir,
    cpu: Cpu,
}

impl Robot {
    pub fn new(pos: Coord, prog: &Vec<i64>) -> Self {
        let mut cpu = Cpu::new(prog);
        cpu.set_print_output(false);
        Robot {
            pos: pos,
            dir: Dir::Up,
            cpu: cpu,
        }
    }

    pub fn get_pos(&self) -> Coord {
        self.pos
    }

    pub fn move_fwd(&mut self) {
        self.pos = match self.dir {
            Dir::Up => Coord { x: self.pos.x, y: self.pos.y - 1 },
            Dir::Down =>  Coord { x: self.pos.x, y: self.pos.y + 1 },
            Dir::Left => Coord { x: self.pos.x - 1, y: self.pos.y },
            Dir::Right => Coord { x: self.pos.x + 1, y: self.pos.y },
        }
    }

    pub fn step_with_input(&mut self, color: TileColor) -> (bool, TileColor) {
        self.cpu.add_input(color.to_num());
        self.cpu.exec_prog();

        let color = TileColor::from_num(self.cpu.pop_output().unwrap());
        let turn_dir = TurnDir::from_num(self.cpu.pop_output().unwrap());

        self.dir = self.dir.turn(turn_dir);
        self.move_fwd();

        (self.cpu.get_state() != CpuState::Done, color)
    }
}

pub fn do_step(grid: &mut Grid, robot: &mut Robot) -> bool {
    let start_pos = robot.get_pos();
    let start_color = match grid.get(&start_pos) {
        None => TileColor::Black,
        Some(color) => *color,
    };
    let (cont, end_color) = robot.step_with_input(start_color);
    grid.insert(start_pos, end_color);
    cont
}

pub fn run_robot_sim(prog: &Vec<i64>, start_on_white: bool) -> Grid {
    let mut grid = HashMap::new();
    if start_on_white {
        grid.insert(Coord { x: 0, y: 0 }, TileColor::White);
    }
    let mut robot = Robot::new(Coord { x: 0, y: 0 }, &prog);
    while do_step(&mut grid, &mut robot) {}
    grid
}

pub fn print_grid(grid: &Grid) {
    let min_coord = Coord { x: i32::max_value(), y: i32::max_value() };
    let max_coord = Coord { x: i32::min_value(), y: i32::min_value() };
    let (min_coord, max_coord) = grid.keys()
        .fold((min_coord, max_coord), |(min_coord, max_coord), coord| {
            let min_x = std::cmp::min(min_coord.x, coord.x);
            let min_y = std::cmp::min(min_coord.y, coord.y);
            let max_x = std::cmp::max(max_coord.x, coord.x);
            let max_y = std::cmp::max(max_coord.y, coord.y);
            let min_coord = Coord { x: min_x, y: min_y };
            let max_coord = Coord { x: max_x, y: max_y };
            (min_coord, max_coord)
        });

    println!("{:?} -> {:?}", min_coord, max_coord);
    (min_coord.y..=max_coord.y).for_each(|y| {
        (min_coord.x..=max_coord.x).for_each(|x| {
            if let Some(color) = grid.get(&Coord { x: x, y: y }) {
                if *color == TileColor::White {
                    print!("#");
                } else {
                    print!(" ");
                }
            } else {
                print!(" ");
            }
        });
        println!("");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_dir_from_num() {
        assert_eq!(TurnDir::from_num(0), TurnDir::Left);
        assert_eq!(TurnDir::from_num(1), TurnDir::Right);
    }

    #[test]
    fn test_dir_turn() {
        assert_eq!(Dir::Up.turn(TurnDir::Left), Dir::Left);
        assert_eq!(Dir::Up.turn(TurnDir::Right), Dir::Right);
        assert_eq!(Dir::Down.turn(TurnDir::Left), Dir::Right);
        assert_eq!(Dir::Down.turn(TurnDir::Right), Dir::Left);
        assert_eq!(Dir::Left.turn(TurnDir::Left), Dir::Down);
        assert_eq!(Dir::Left.turn(TurnDir::Right), Dir::Up);
        assert_eq!(Dir::Right.turn(TurnDir::Left), Dir::Up);
        assert_eq!(Dir::Right.turn(TurnDir::Right), Dir::Down);
    }

    #[test]
    fn test_tile_color_from_num() {
        assert_eq!(TileColor::from_num(0), TileColor::Black);
        assert_eq!(TileColor::from_num(1), TileColor::White);
    }

    #[test]
    fn test_tile_color_to_num() {
        assert_eq!(TileColor::Black.to_num(), 0);
        assert_eq!(TileColor::White.to_num(), 1);
    }

    #[test]
    fn test_new_robot() {
        let result = Robot::new(Coord { x: 5, y: 6}, &vec![]);
        assert_eq!(result.pos, Coord { x: 5, y: 6});
    }
}
