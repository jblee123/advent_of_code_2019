use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

use aoc2019_utils::*;

use crate::day15_cpu::*;

pub type Coord = point_2d::Point2d<i16>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile { Space, Wall, Oxygen, Unknown }

pub struct ShipMap {
    pub tiles: HashMap<Coord, Tile>,
    pub top: i16,
    pub left: i16,
    pub bottom: i16,
    pub right: i16,
}

impl ShipMap {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            top: 0,
            left: 0,
            bottom: 0,
            right: 0,
        }
    }

    fn update_tile(&mut self, coord: Coord, tile: Tile) {
        self.tiles.insert(coord, tile);

        self.top = std::cmp::min(self.top, coord.y);
        self.bottom = std::cmp::max(self.bottom, coord.y);
        self.left = std::cmp::min(self.left, coord.x);
        self.right = std::cmp::max(self.right, coord.x);
    }

    fn get_tile_at(&self, coord: Coord) -> Tile {
        match self.tiles.get(&coord) {
            None => Tile::Unknown,
            Some(tile) => *tile,
        }
    }

    pub fn get_oxygen_pos(&self) -> Option<Coord> {
        self.tiles.iter().filter(|(_, tile)| {
            **tile == Tile::Oxygen
        }).map(|(pos, _)| {
            *pos
        }).next()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir { North, South, West, East }

impl Dir {
    fn to_num(self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }
}

fn move_coord(coord: Coord, dir: Dir) -> Coord {
    let mut result = coord;
    match dir {
        Dir::North => result.y -= 1,
        Dir::South => result.y += 1,
        Dir::West => result.x -= 1,
        Dir::East => result.x += 1,
    }
    result
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MoveResult { HitWall, Moved, OnOxygen }

impl MoveResult {
    fn from_num(num: i64) -> Self {
        match num {
            0 => Self::HitWall,
            1 => Self::Moved,
            2 => Self::OnOxygen,
            _ => panic!("bad result num"),
        }
    }
}

pub struct Robot {
    cpu: Cpu,
    pos: Coord,
}

impl Robot {
    fn new(prog: &Vec<i64>) -> Self {
        let mut cpu = Cpu::new(prog);
        cpu.set_print_output(false);
        Robot {
            cpu: cpu,
            pos: Coord { x: 0, y: 0 },
        }
    }

    fn take_step(&mut self, dir: Dir) -> MoveResult {
        self.cpu.add_input(dir.to_num());
        self.cpu.exec_prog();
        let output = self.cpu.pop_output().unwrap();
        let result = MoveResult::from_num(output);

        if result != MoveResult::HitWall {
            self.pos = move_coord(self.pos, dir);
        }

        result
    }
}

pub fn find_longest_path_len(
    ship_map: &ShipMap,
    from_pos: Coord,
) -> u32 {
    struct CoordAndDist {
        pos: Coord,
        dist: u32,
    }

    let mut to_visit = LinkedList::new();
    let mut visited = HashSet::new();
    to_visit.push_back(
        CoordAndDist {
            pos: from_pos,
            dist: 0,
        }
    );

    let get_next_visits = |at, visited: &HashSet<Coord>| -> Vec<CoordAndDist> {
        [Dir::North, Dir::South, Dir::East, Dir::West].iter()
            .map(|dir| {
                CoordAndDist {
                    pos: move_coord(at, *dir),
                    dist: 0,
                }
            })
            .filter(|coord_and_dist| {
                !visited.contains(&coord_and_dist.pos)
                    && ship_map.get_tile_at(coord_and_dist.pos) != Tile::Wall
            })
            .collect()
    };

    let mut max_dist = 0;

    loop {
        if to_visit.is_empty() {
            break;
        }

        let at = to_visit.pop_front().unwrap();
        max_dist = at.dist;

        visited.insert(at.pos);

        get_next_visits(at.pos, &visited).into_iter().for_each(|mut visit| {
            visit.dist = at.dist + 1;
            to_visit.push_back(visit);
        });
    }

    max_dist
}

pub fn find_shortest_path_len(
    ship_map: &ShipMap,
    from_pos: Coord,
    to_pos: Coord,
) -> Option<u32> {
    struct CoordAndDist {
        pos: Coord,
        dist: u32,
    }

    let mut to_visit = LinkedList::new();
    let mut visited = HashSet::new();
    to_visit.push_back(
        CoordAndDist {
            pos: from_pos,
            dist: 0,
        }
    );

    let get_next_visits = |at, visited: &HashSet<Coord>| -> Vec<CoordAndDist> {
        [Dir::North, Dir::South, Dir::East, Dir::West].iter()
            .map(|dir| {
                CoordAndDist {
                    pos: move_coord(at, *dir),
                    dist: 0,
                }
            })
            .filter(|coord_and_dist| {
                !visited.contains(&coord_and_dist.pos)
                    && ship_map.get_tile_at(coord_and_dist.pos) != Tile::Wall
            })
            .collect()
    };

    loop {
        if to_visit.is_empty() {
            break;
        }

        let at = to_visit.pop_front().unwrap();
        if at.pos == to_pos {
            return Some(at.dist);
        }

        visited.insert(at.pos);

        get_next_visits(at.pos, &visited).into_iter().for_each(|mut visit| {
            visit.dist = at.dist + 1;
            to_visit.push_back(visit);
        });
    }

    None
}

fn find_closest_unknown_tile_dir(ship_map: &ShipMap, start_pos: Coord)
-> Option<Dir> {
    struct CoordAndDir {
        pos: Coord,
        dir: Dir,
    }

    let mut to_visit = LinkedList::new();
    let mut visited = HashSet::new();
    visited.insert(start_pos);

    let get_next_visits = |at, visited: &HashSet<Coord>| -> Vec<CoordAndDir> {
        [Dir::North, Dir::South, Dir::East, Dir::West].iter()
            .map(|dir| {
                CoordAndDir {
                    pos: move_coord(at, *dir),
                    dir: *dir,
                }
            })
            .filter(|coord_and_dir| {
                !visited.contains(&coord_and_dir.pos)
                    && ship_map.get_tile_at(coord_and_dir.pos) != Tile::Wall
            })
            .collect()
    };

    get_next_visits(start_pos, &visited).into_iter().for_each(|visit| {
        to_visit.push_back(visit);
    });

    loop {
        if to_visit.is_empty() {
            break;
        }

        let at = to_visit.pop_front().unwrap();
        if ship_map.get_tile_at(at.pos) == Tile::Unknown {
            return Some(at.dir);
        }

        visited.insert(at.pos);

        get_next_visits(at.pos, &visited).into_iter().for_each(|mut visit| {
            visit.dir = at.dir;
            to_visit.push_back(visit);
        });
    }

    None
}

pub fn create_map(prog: &Vec<i64>) -> (ShipMap, Coord) {
    let mut robot = Robot::new(prog);
    let start_pos = robot.pos;

    let mut ship_map = ShipMap::new();
    ship_map.update_tile(start_pos, Tile::Space);

    while let Some(dir) = find_closest_unknown_tile_dir(&ship_map, robot.pos) {
        let (tile, pos) = match robot.take_step(dir) {
            MoveResult::HitWall => (Tile::Wall, move_coord(robot.pos, dir)),
            MoveResult::Moved => (Tile::Space, robot.pos),
            MoveResult::OnOxygen => (Tile::Oxygen, robot.pos),
        };
        ship_map.update_tile(pos, tile);
    }

    (ship_map, start_pos)
}

pub fn print_map(ship_map: &ShipMap) {
    println!("");
    (ship_map.top..=ship_map.bottom).for_each(|y|{
        (ship_map.left..=ship_map.right).for_each(|x|{
            let c = match ship_map.get_tile_at(Coord { x: x, y: y }) {
                Tile::Space => ' ',
                Tile::Wall => '#',
                Tile::Oxygen => 'O',
                Tile::Unknown => '.',
            };
            print!("{}", c);
        });
        println!("");
    });
}
