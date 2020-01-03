use aoc2019_utils::*;

// use crate::day17_cpu::*;

pub type Coord = point_2d::Point2d<u8>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '>' => Self::Left,
            '<' => Self::Right,
            _ => panic!("bad char for Dir: {}", c),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '>',
            Self::Right => '<',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pose {
    pos: Coord,
    dir: Dir,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile { Space, Scaffold }

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' | '^' | 'v' | '>' | '<' => Self::Scaffold,
            '.' => Self::Space,
            _ => panic!("bad tile char: '{}'", c),
        }
    }
}

pub type ScafMap = Vec<Vec<Tile>>;

pub fn parse_map_from_robot(to_parse: &Vec<i64>) -> (ScafMap, Pose) {
    let width = to_parse.iter().position(|c| *c == ('\n' as i64)).unwrap();
    let height = to_parse.iter()
        .fold(0, |acc, c| if *c == ('\n' as i64) { acc + 1 } else { acc }) - 1;
    println!("width/height: {} / {}", width, height);

    let mut new_scaf_map = vec![vec![Tile::Space; height]; width];
    let mut robot_pose = Pose {
        pos: Coord { x: 0, y: 0 },
        dir: Dir::Up,
    };

    for y in 0..height {
        for x in 0..width {
            let idx = (y * (width + 1)) + x;
            let c = to_parse[idx] as u8 as char;
            new_scaf_map[x][y] = Tile::from_char(c);
            match c {
                '^' | 'v' | '>' | '<' => {
                    robot_pose = Pose {
                        pos: Coord { x: x as u8, y: y as u8 },
                        dir: Dir::from_char(c as char),
                    };
                },
                _ => {},
            };
        }
    }

    (new_scaf_map, robot_pose)
}

pub fn print_map(scaf_map: &ScafMap, pose: &Pose) {
    let width = scaf_map.len();
    let height = scaf_map[0].len();

    for y in 0..height {
        for x in 0..width {
            if pose.pos == (Coord { x: x as u8, y: y as u8 }) {
                print!("{}", pose.dir.to_char());
            } else {
                let c = match scaf_map[x][y] {
                    Tile::Space => '.',
                    Tile::Scaffold => '#',
                };
                print!("{}", c);
            }
        }
        println!("");
    }
}

pub fn ascii_to_vec(s: &str) -> Vec<i64> {
    s.bytes().map(|c| c as i64).collect()
}

pub fn get_alignment_param(scaf_map: &ScafMap) -> u32 {
    let width = scaf_map.len();
    let height = scaf_map[0].len();

    let mut alignment_param = 0;
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let tile = scaf_map[x][y];
            if tile == Tile::Space {
                continue;
            }

            let is_intersection =
                scaf_map[x - 1][y] != Tile::Space
                && scaf_map[x + 1][y] != Tile::Space
                && scaf_map[x][y - 1] != Tile::Space
                && scaf_map[x][y + 1] != Tile::Space
                && scaf_map[x - 1][y - 1] == Tile::Space
                && scaf_map[x + 1][y - 1] == Tile::Space
                && scaf_map[x - 1][y + 1] == Tile::Space
                && scaf_map[x + 1][y + 1] == Tile::Space;

            if is_intersection {
                alignment_param += x * y;
            }
        }
    }

    alignment_param as u32
}
