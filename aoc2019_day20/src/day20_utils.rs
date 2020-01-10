use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;

use aoc2019_utils::*;

pub type Coord = point_2d::Point2d<u8>;

const ZERO_COORD: Coord = Coord { x: 0, y: 0 };

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct PortalDesc {
    dest: Coord,
    is_outer: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tile {
    None,
    Space,
    Wall,
    Portal(PortalDesc),
}

pub type MazeTiles = Vec<Vec<Tile>>;

pub struct Maze {
    tiles: MazeTiles,
    start: Coord,
    end: Coord,
}

fn to_portal_tile(dest: Coord, is_outer: bool) -> Tile {
    Tile::Portal(
        PortalDesc {
            dest: dest,
            is_outer: is_outer,
        })
}

fn parse_maze_marker(
    lines: &Vec<&str>,
    x: usize,
    y: usize,
    width: usize,
    height: usize
) -> Option<(u8, u8, Coord, Coord, bool)> {
    let get_char = |x: usize, y: usize| { lines[y].as_bytes()[x] };
    let is_char = |c| { b'A' <= c && c <= b'Z' };
    let to_coord = |x: usize, y: usize| Coord { x: x as u8, y: y as u8 };

    if ((x > 0) && is_char(get_char(x - 1, y)))
        || ((y > 0) && is_char(get_char(x, y - 1)))
    {
        return None;
    }

    let c1 = get_char(x, y);

    let c2 = {
        let c = get_char(x + 1, y);
        if is_char(c) {
            c
        } else {
            get_char(x, y + 1)
        }
    };

    let send_coord;
    let recv_coord;
    let outer_tile;

    if x == 0 {
        send_coord = to_coord(1, y);
        recv_coord = to_coord(2, y);
        outer_tile = true;
    } else if y == 0 {
        send_coord = to_coord(x, 1);
        recv_coord = to_coord(x, 2);
        outer_tile = true;
    } else if x == (width - 2) {
        send_coord = to_coord(width - 2, y);
        recv_coord = to_coord(width - 3, y);
        outer_tile = true;
    } else if y == (height - 2) {
        send_coord = to_coord(x, height - 2);
        recv_coord = to_coord(x, height - 3);
        outer_tile = true;
    } else if get_char(x - 1, y) == b'.' {
        send_coord = to_coord(x, y);
        recv_coord = to_coord(x - 1, y);
        outer_tile = false;
    } else if get_char(x, y - 1) == b'.' {
        send_coord = to_coord(x, y);
        recv_coord = to_coord(x, y - 1);
        outer_tile = false;
    } else if is_char(get_char(x + 1, y)) {
        send_coord = to_coord(x + 1, y);
        recv_coord = to_coord(x + 2, y);
        outer_tile = false;
    } else if is_char(get_char(x, y + 1)) {
        send_coord = to_coord(x, y + 1);
        recv_coord = to_coord(x, y + 2);
        outer_tile = false;
    } else {
        panic!("something fishy parsing at coord {}, {}", x, y);
    }

    Some((c1, c2, send_coord, recv_coord, outer_tile))
}

pub fn parse_input(input: &str) -> Maze {
    let lines = input.lines().collect::<Vec<&str>>();

    let height = lines.len();
    let width = lines[0].len();

    let mut maze = Maze {
        tiles: vec![vec![Tile::None; height]; width],
        start: ZERO_COORD,
        end: ZERO_COORD,
    };

    let mut portals: HashMap<(u8, u8), (Coord, Coord, bool)> = HashMap::new();

    let mut handle_char_at = |x: usize, y: usize| {
        let c = lines[y].as_bytes()[x];
        match c {
            b'#' => maze.tiles[x][y] = Tile::Wall,
            b'.' => maze.tiles[x][y] = Tile::Space,
            b'A'..=b'Z' => {
                match parse_maze_marker(&lines, x, y, width, height) {
                    Some((c1, c2, send_coord, recv_coord, is_outer)) => {
                        if c1 == b'A' && c2 == b'A' {
                            maze.start = recv_coord;
                        } else if c1 == b'Z' && c2 == b'Z' {
                            maze.end = recv_coord;
                        } else if let Some((other_send, other_recv, other_is_outer)) =
                            portals.remove(&(c1, c2))
                        {
                            maze.tiles[send_coord.x as usize][send_coord.y as usize] =
                                to_portal_tile(other_recv, is_outer);
                            maze.tiles[other_send.x as usize][other_send.y as usize] =
                                to_portal_tile(recv_coord, other_is_outer);
                        } else {
                            portals.insert(
                                (c1, c2),
                                (send_coord, recv_coord, is_outer)
                            );
                        }
                    },
                    None => {},
                }
            },
            b' ' => {},
            _ => panic!("Bad maze character '{}' at ({}, {})", c as char, x, y),
        }
    };

    for x in 0..width {
        for y in 0..height {
            handle_char_at(x, y);
        }
    }

    maze
}

fn get_reachable_coords(pos: Coord, width: usize, height: usize) -> Vec<Coord> {
    let mut coords = Vec::with_capacity(4);
    if pos.y > 0 {
        coords.push(Coord { x: pos.x, y: pos.y - 1 });
    }
    if pos.y < height as u8 - 1 {
        coords.push(Coord { x: pos.x, y: pos.y + 1 });
    }
    if pos.x > 0 {
        coords.push(Coord { x: pos.x - 1, y: pos.y });
    }
    if pos.x < width as u8 - 1 {
        coords.push(Coord { x: pos.x + 1, y: pos.y });
    }

    coords
}

pub fn traverse_maze(maze: &Maze) -> Option<u32> {

    struct SearchState {
        loc: Coord,
        num_steps: u32,
    }

    let width = maze.tiles.len();
    let height = maze.tiles[0].len();

    let mut history = HashSet::new();
    let mut to_visit = LinkedList::new();

    to_visit.push_back(
        SearchState {
            loc: maze.start,
            num_steps: 0,
        }
    );

    loop {
        if to_visit.is_empty() {
            return None;
        }

        let cur_state = to_visit.pop_front().unwrap();
        if cur_state.loc == maze.end {
            return Some(cur_state.num_steps);
        }

        history.insert(cur_state.loc.clone());

        let next_coords = get_reachable_coords(cur_state.loc, width, height);
        to_visit.extend(next_coords.into_iter()
            .filter_map(|c| {
                match maze.tiles[c.x as usize][c.y as usize] {
                    Tile::Space => Some(c),
                    Tile::Portal(desc) => Some(desc.dest),
                    _ => None,
                }
            })
            .filter(|c| !history.contains(&c))
            .map(|c| SearchState {
                loc: c,
                num_steps: cur_state.num_steps + 1,
            })
        );
    }
}

pub fn traverse_recursive_maze(maze: &Maze) -> Option<u32> {

    #[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
    struct SearchVisit {
        loc: Coord,
        depth: u16,
    }

    struct SearchState {
        loc: Coord,
        depth: u16,
        num_steps: u32,
    }

    let width = maze.tiles.len();
    let height = maze.tiles[0].len();

    let mut history = HashSet::new();
    let mut to_visit = LinkedList::new();

    to_visit.push_back(
        SearchState {
            loc: maze.start,
            depth: 0,
            num_steps: 0,
        }
    );

    loop {
        if to_visit.is_empty() {
            return None;
        }

        let cur_state = to_visit.pop_front().unwrap();
        if (cur_state.loc == maze.end) && (cur_state.depth == 0) {
            return Some(cur_state.num_steps);
        }

        history.insert(
            SearchVisit {
                loc: cur_state.loc,
                depth: cur_state.depth,
            }
        );

        let next_coords = get_reachable_coords(cur_state.loc, width, height);
        to_visit.extend(next_coords.into_iter()
            .filter_map(|c| {
                let tile = maze.tiles[c.x as usize][c.y as usize];

                if cur_state.depth == 0 {
                    if let Tile::Portal(desc) = tile {
                        if desc.is_outer {
                            return None;
                        }
                    }
                } else if c == maze.start || c == maze.end {
                    return None;
                }

                match tile {
                    Tile::Space => Some(
                        SearchVisit {
                            loc: c,
                            depth: cur_state.depth,
                        }
                    ),
                    Tile::Portal(desc) => {
                        let new_depth = if desc.is_outer {
                            cur_state.depth - 1
                        } else {
                            cur_state.depth + 1
                        };
                        Some(
                            SearchVisit {
                                loc: desc.dest,
                                depth: new_depth,
                            }
                        )
                    },
                    _ => None,
                }
            })
            .filter(|search_visit| !history.contains(&search_visit))
            .map(|search_visit| SearchState {
                loc: search_visit.loc,
                depth: search_visit.depth,
                num_steps: cur_state.num_steps + 1,
            })
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_MAZE_1: &str = concat!(
        "         A           \n",
        "         A           \n",
        "  #######.#########  \n",
        "  #######.........#  \n",
        "  #######.#######.#  \n",
        "  #######.#######.#  \n",
        "  #######.#######.#  \n",
        "  #####  B    ###.#  \n",
        "BC...##  C    ###.#  \n",
        "  ##.##       ###.#  \n",
        "  ##...DE  F  ###.#  \n",
        "  #####    G  ###.#  \n",
        "  #########.#####.#  \n",
        "DE..#######...###.#  \n",
        "  #.#########.###.#  \n",
        "FG..#########.....#  \n",
        "  ###########.#####  \n",
        "             Z       \n",
        "             Z       \n",
    );

    const SAMPLE_MAZE_2: &str = concat!(
        "                   A               \n",
        "                   A               \n",
        "  #################.#############  \n",
        "  #.#...#...................#.#.#  \n",
        "  #.#.#.###.###.###.#########.#.#  \n",
        "  #.#.#.......#...#.....#.#.#...#  \n",
        "  #.#########.###.#####.#.#.###.#  \n",
        "  #.............#.#.....#.......#  \n",
        "  ###.###########.###.#####.#.#.#  \n",
        "  #.....#        A   C    #.#.#.#  \n",
        "  #######        S   P    #####.#  \n",
        "  #.#...#                 #......VT\n",
        "  #.#.#.#                 #.#####  \n",
        "  #...#.#               YN....#.#  \n",
        "  #.###.#                 #####.#  \n",
        "DI....#.#                 #.....#  \n",
        "  #####.#                 #.###.#  \n",
        "ZZ......#               QG....#..AS\n",
        "  ###.###                 #######  \n",
        "JO..#.#.#                 #.....#  \n",
        "  #.#.#.#                 ###.#.#  \n",
        "  #...#..DI             BU....#..LF\n",
        "  #####.#                 #.#####  \n",
        "YN......#               VT..#....QG\n",
        "  #.###.#                 #.###.#  \n",
        "  #.#...#                 #.....#  \n",
        "  ###.###    J L     J    #.#.###  \n",
        "  #.....#    O F     P    #.#...#  \n",
        "  #.###.#####.#.#####.#####.###.#  \n",
        "  #...#.#.#...#.....#.....#.#...#  \n",
        "  #.#####.###.###.#.#.#########.#  \n",
        "  #...#.#.....#...#.#.#.#.....#.#  \n",
        "  #.###.#####.###.###.#.#.#######  \n",
        "  #.#.........#...#.............#  \n",
        "  #########.###.###.#############  \n",
        "           B   J   C               \n",
        "           U   P   P               \n",
    );

    const SAMPLE_MAZE_3: &str = concat!(
        "             Z L X W       C                 \n",
        "             Z P Q B       K                 \n",
        "  ###########.#.#.#.#######.###############  \n",
        "  #...#.......#.#.......#.#.......#.#.#...#  \n",
        "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n",
        "  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n",
        "  #.###.#######.###.###.#.###.###.#.#######  \n",
        "  #...#.......#.#...#...#.............#...#  \n",
        "  #.#########.#######.#.#######.#######.###  \n",
        "  #...#.#    F       R I       Z    #.#.#.#  \n",
        "  #.###.#    D       E C       H    #.#.#.#  \n",
        "  #.#...#                           #...#.#  \n",
        "  #.###.#                           #.###.#  \n",
        "  #.#....OA                       WB..#.#..ZH\n",
        "  #.###.#                           #.#.#.#  \n",
        "CJ......#                           #.....#  \n",
        "  #######                           #######  \n",
        "  #.#....CK                         #......IC\n",
        "  #.###.#                           #.###.#  \n",
        "  #.....#                           #...#.#  \n",
        "  ###.###                           #.#.#.#  \n",
        "XF....#.#                         RF..#.#.#  \n",
        "  #####.#                           #######  \n",
        "  #......CJ                       NM..#...#  \n",
        "  ###.#.#                           #.###.#  \n",
        "RE....#.#                           #......RF\n",
        "  ###.###        X   X       L      #.#.#.#  \n",
        "  #.....#        F   Q       P      #.#.#.#  \n",
        "  ###.###########.###.#######.#########.###  \n",
        "  #.....#...#.....#.......#...#.....#.#...#  \n",
        "  #####.#.###.#######.#######.###.###.#.#.#  \n",
        "  #.......#.......#.#.#.#.#...#...#...#.#.#  \n",
        "  #####.###.#####.#.#.#.#.###.###.#.###.###  \n",
        "  #.......#.....#.#...#...............#...#  \n",
        "  #############.#.#.###.###################  \n",
        "               A O F   N                     \n",
        "               A A D   M                     \n",
    );

    #[test]
    fn test_parse_input() {
        let maze = parse_input(SAMPLE_MAZE_1);
        assert_eq!(maze.start, Coord { x: 9, y: 2 });
        assert_eq!(maze.end, Coord { x: 13, y: 16 });

        let to_portal_tile_xy = |x, y, is_outer| {
            to_portal_tile(Coord { x: x, y: y }, is_outer)
        };

        assert_eq!(maze.tiles[9][7], to_portal_tile_xy(2, 8, false));
        assert_eq!(maze.tiles[1][8], to_portal_tile_xy(9, 6, true));

        assert_eq!(maze.tiles[7][10], to_portal_tile_xy(2, 13, false));
        assert_eq!(maze.tiles[1][13], to_portal_tile_xy(6, 10, true));

        assert_eq!(maze.tiles[11][11], to_portal_tile_xy(2, 15, false));
        assert_eq!(maze.tiles[1][15], to_portal_tile_xy(11, 12, true));


        let maze = parse_input(SAMPLE_MAZE_2);
        assert_eq!(maze.start, Coord { x: 19, y: 2 });
        assert_eq!(maze.end, Coord { x: 2, y: 17 });
    }

    #[test]
    fn test_traverse_maze() {
        let maze = parse_input(SAMPLE_MAZE_1);
        let result = traverse_maze(&maze);
        assert_eq!(result, Some(23));

        let maze = parse_input(SAMPLE_MAZE_2);
        let result = traverse_maze(&maze);
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_traverse_recursive_maze() {
        let maze = parse_input(SAMPLE_MAZE_1);
        let result = traverse_recursive_maze(&maze);
        assert_eq!(result, Some(26));

        let maze = parse_input(SAMPLE_MAZE_3);
        let result = traverse_recursive_maze(&maze);
        assert_eq!(result, Some(396));
    }
}
