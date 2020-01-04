use std::collections::HashSet;
use std::collections::LinkedList;
use std::time::Instant;

use aoc2019_utils::*;

const WALL: char = '#';
const SPACE: char = '.';

pub type Coord = point_2d::Point2d<u8>;

const ZERO_COORD: Coord = Coord { x: 0, y: 0 };

pub type Vault = Vec<Vec<char>>;

fn vault_cell_at(vault: &Vault, coord: Coord) -> char {
    vault[coord.x as usize][coord.y as usize]
}

fn set_vault_cell_at(vault: &mut Vault, coord: Coord, c: char) {
    vault[coord.x as usize][coord.y as usize] = c;
}

fn key_letter_to_bit(c: char) -> u32 {
    1 << (c as u8 - b'a') as u32
}

fn get_num_keys(keys: u32) -> u32 {
    let mut keys = keys;
    let mut num_keys = 0;
    while keys > 0 {
        if (keys & 1) == 1 {
            num_keys += 1;
        }
        keys >>= 1;
    }
    num_keys
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

pub fn print_vault(vault: &Vault) {
    let width = vault.len();
    let height = vault[0].len();
    for y in 0..height {
        for x in 0..width {
            print!("{}", vault[x][y]);
        }
        println!("");
    }
}

pub fn parse_input(input: &str) -> (Vault, Coord, u32) {
    let lines = input.lines().collect::<Vec<&str>>();

    let height = lines.len();
    let width = lines[0].len();

    let mut pos = ZERO_COORD;
    let mut keys = 0;

    let mut vault = vec![vec![SPACE; height]; width];
    for x in 0..width {
        for y in 0..height {
            let c = lines[y].as_bytes()[x] as char;
            vault[x][y] = c;
            match c {
                '@' => pos = Coord { x: x as u8, y: y as u8 },
                'a'..='z' => keys |= key_letter_to_bit(c),
                _ => {},
            }
        }
    }

    (vault, pos, keys)
}

pub fn replace_vault_center(vault: &Vault, start_pos: Coord)
-> (Vault, Vec<Coord>) {

    assert_eq!(vault_cell_at(vault, start_pos), '@');

    let mut vault = vault.clone();

    let left = start_pos.x - 1;
    let right = start_pos.x + 1;
    let top = start_pos.y - 1;
    let bottom = start_pos.y + 1;

    let new_start_positions = vec![
        Coord { x: left, y: top },
        Coord { x: right, y: top },
        Coord { x: left, y: bottom },
        Coord { x: right, y: bottom },
    ];

    for x in left..=right {
        for y in top..=bottom {
            vault[x as usize][y as usize] = '#';
        }
    }

    for pos in &new_start_positions {
        set_vault_cell_at(&mut vault, *pos, '@');
    }

    (vault, new_start_positions)
}

pub fn process_vault(vault: &Vault, start_pos: Coord) -> Vault {
    let width = vault.len();
    let height = vault[0].len();

    let mut new_vault = vault.clone();

    let mut depth_state = vec![start_pos];
    let mut unavail_as_child = HashSet::new();
    unavail_as_child.insert(start_pos);

    while !depth_state.is_empty() {
        let cur_pos = *depth_state.last().unwrap();
        let reachable = get_reachable_coords(cur_pos, width, height);
        let children = reachable.iter()
            .filter(|pos| {
                if vault_cell_at(&new_vault, **pos) == WALL {
                    return false;
                }
                if unavail_as_child.contains(pos) {
                    return false;
                }

                true
            })
            .cloned()
            .collect::<Vec<Coord>>();

        if !children.is_empty() {
            unavail_as_child.extend(children.iter());
            depth_state.extend(children.iter());
            continue;
        }

        depth_state.pop();
        if vault_cell_at(&new_vault, cur_pos) != SPACE {
            continue;
        }

        let num_surrounding_walls = reachable.iter()
            .filter(|pos| vault_cell_at(&new_vault, **pos) == WALL)
            .count();

        if num_surrounding_walls >= 3 {
            set_vault_cell_at(&mut new_vault, cur_pos, WALL);
        }
    }

    new_vault
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct KeySearchVisitation {
    positions: [Coord; 4],
    keys: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct KeySearchState {
    visitation: KeySearchVisitation,
    num_steps: u32,
}

fn generate_next_states_for_key_search(
    start_state: &KeySearchState,
    vault: &Vault,
    history: &HashSet<KeySearchVisitation>,
    num_robots: usize,
) -> Vec<KeySearchState> {
    let width = vault.len();
    let height = vault[0].len();
    let cur_keys = start_state.visitation.keys;

    #[derive(Debug)]
    struct CoordWithNum {
        coord: Coord,
        robot_num: usize,
    }

    let mut next_coords = vec![];

    for i in 0..num_robots {
        next_coords.extend(
            get_reachable_coords(start_state.visitation.positions[i], width, height)
                .into_iter()
                .map(|coord| CoordWithNum { coord: coord, robot_num: i })
        );
    }

    let mut next_states = Vec::with_capacity(16);

    for next_coord in next_coords {
        let c = vault_cell_at(vault, next_coord.coord);

        if c == WALL {
            continue;
        }

        if ('A'..='Z').contains(&c) {
            let lower_c = c.to_lowercase().next().unwrap();
            if (cur_keys & key_letter_to_bit(lower_c)) == 0 {
                continue;
            }
        }

        let new_keys = match c {
            'a'..='z' => {
                cur_keys | key_letter_to_bit(c)
            },
            '.' | '@' | 'A'..='Z' => {
                cur_keys
            },
            _ => panic!("unknown char in vault: {}", c),
        };

        let mut new_visitation = start_state.visitation;
        new_visitation.positions[next_coord.robot_num] = next_coord.coord;
        new_visitation.keys = new_keys;

        if history.contains(&new_visitation) {
            continue;
        }

        next_states.push(
            KeySearchState {
                visitation: new_visitation,
                num_steps: start_state.num_steps + 1
            }
        );
    }

    next_states
}

pub fn search_for_keys(
    start_positions: &Vec<Coord>,
    vault: &Vault,
    all_keys: u32,
) -> Option<u32> {

    let mut vault = vault.clone();
    for start_pos in start_positions {
        vault = process_vault(&vault, *start_pos);
    }

    let mut to_visit = LinkedList::new();
    let mut history = HashSet::new();

    let mut first_visitation = KeySearchVisitation {
        positions: [ZERO_COORD; 4],
        keys: 0
    };
    for (i, start_pos) in start_positions.iter().enumerate() {
        first_visitation.positions[i] = *start_pos;
    }

    let first_state =  KeySearchState {
        visitation: first_visitation,
        num_steps: 0,
    };
    to_visit.push_back(first_state);

    let start_time = Instant::now();
    let mut last_elapsed_sec = 0;
    loop {
        if to_visit.is_empty() {
            break None
        }

        let visiting = to_visit.pop_front().unwrap();

        if visiting.visitation.keys == all_keys {
            break Some(visiting.num_steps)
        }

        let cur_elapsed_sec = start_time.elapsed().as_secs();
        let sec_since_msg = cur_elapsed_sec - last_elapsed_sec;
        if sec_since_msg >= 5 {
            let num_keys = get_num_keys(visiting.visitation.keys);
            let msg_time = sec_to_hrs_mins_secs_str(cur_elapsed_sec);
            println!(
                "keys: {}, to visit len: {}, history len: {}, elapsed: {}",
                num_keys, to_visit.len(), history.len(), msg_time,
            );
            last_elapsed_sec = cur_elapsed_sec;
        }

        history.insert(visiting.visitation);
        let next_states = generate_next_states_for_key_search(
            &visiting,
            &vault,
            &history,
            start_positions.len(),
        );
        to_visit.extend(next_states.into_iter());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        const SAMPLE_INPUT_1: &str = concat!(
            "#########\n",
            "#b.A.@.a#\n",
            "#########\n",
        );

        let target_vault = vec![
            vec!['#', '#', '#'],
            vec!['#', 'b', '#'],
            vec!['#', '.', '#'],
            vec!['#', 'A', '#'],
            vec!['#', '.', '#'],
            vec!['#', '@', '#'],
            vec!['#', '.', '#'],
            vec!['#', 'a', '#'],
            vec!['#', '#', '#'],
        ];
        let target_pos = Coord { x: 5, y: 1 };
        let target_keys = 0b11;
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_1);
        assert_eq!(vault, target_vault);
        assert_eq!(pos, target_pos);
        assert_eq!(keys, target_keys);
    }

    #[test]
    fn test_process_vault() {
        const MAP_PRE: &str = concat!(
            "########################\n",
            "#....#.#.#.#B#.......#.#\n",
            "#.####.#.#.#.#.#.#.#.#.#\n",
            "#b##.#.#.#.#.###.###.#.#\n",
            "#................#.....#\n",
            "###########.@.##########\n",
            "#......A...............#\n",
            "############.###########\n",
            "#.................a....#\n",
            "########################\n",
        );

        const MAP_POST: &str = concat!(
            "########################\n",
            "############B###########\n",
            "############.###########\n",
            "#b##########.###########\n",
            "#.............##########\n",
            "###########.@.##########\n",
            "#######A......##########\n",
            "############.###########\n",
            "############......a#####\n",
            "########################\n",
        );

        let (target, _, _) = parse_input(MAP_POST);
        let (result, pos, _) = parse_input(MAP_PRE);
        let result = process_vault(&result, pos);

        assert_eq!(result, target);
    }

    #[test]
    fn test_replace_vault_center() {
        const MAP_PRE: &str = concat!(
            "########################\n",
            "#....#.#.#.#B#.......#.#\n",
            "#.####.#.#.#.#.#.#.#.#.#\n",
            "#b##.#.#.#.#.###.###.#.#\n",
            "#................#.....#\n",
            "###########.@.##########\n",
            "#......A...............#\n",
            "############.###########\n",
            "#.................a....#\n",
            "########################\n",
        );

        const MAP_POST: &str = concat!(
            "########################\n",
            "#....#.#.#.#B#.......#.#\n",
            "#.####.#.#.#.#.#.#.#.#.#\n",
            "#b##.#.#.#.#.###.###.#.#\n",
            "#..........@#@...#.....#\n",
            "########################\n",
            "#......A...@#@.........#\n",
            "############.###########\n",
            "#.................a....#\n",
            "########################\n",
        );

        let (target, _, _) = parse_input(MAP_POST);
        let (result, pos, _) = parse_input(MAP_PRE);
        let (result, new_positions) = replace_vault_center(&result, pos);

        assert_eq!(result, target);

        let target_new_positions: HashSet<Coord> = vec![
            Coord { x: pos.x - 1, y: pos.y - 1 },
            Coord { x: pos.x - 1, y: pos.y + 1 },
            Coord { x: pos.x + 1, y: pos.y - 1 },
            Coord { x: pos.x + 1, y: pos.y + 1 },
        ].into_iter().collect();

        let new_positions: HashSet<Coord> = new_positions.into_iter().collect();

        assert_eq!(new_positions, target_new_positions);
    }

    #[test]
    fn test_search_for_keys() {
        use std::time::Instant;

        const SAMPLE_INPUT_1: &str = concat!(
            "#########\n",
            "#b.A.@.a#\n",
            "#########\n",
        );

        const SAMPLE_INPUT_2: &str = concat!(
            "########################\n",
            "#f.D.E.e.C.b.A.@.a.B.c.#\n",
            "######################.#\n",
            "#d.....................#\n",
            "########################\n",
        );

        const SAMPLE_INPUT_3: &str = concat!(
            "########################\n",
            "#...............b.C.D.f#\n",
            "#.######################\n",
            "#.....@.a.B.c.d.A.e.F.g#\n",
            "########################\n",
        );

        const SAMPLE_INPUT_4: &str = concat!(
            "#################\n",
            "#i.G..c...e..H.p#\n",
            "########.########\n",
            "#j.A..b...f..D.o#\n",
            "########@########\n",
            "#k.E..a...g..B.n#\n",
            "########.########\n",
            "#l.F..d...h..C.m#\n",
            "#################\n",
        );

        const SAMPLE_INPUT_5: &str = concat!(
            "########################\n",
            "#@..............ac.GI.b#\n",
            "###d#e#f################\n",
            "###A#B#C################\n",
            "###g#h#i################\n",
            "########################\n",
        );

        let mut test_num = 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_1);
        let result = search_for_keys(&vec![pos], &vault, keys);
        assert_eq!(result, Some(8));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_2);
        let result = search_for_keys(&vec![pos], &vault, keys);
        assert_eq!(result, Some(86));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_3);
        let result = search_for_keys(&vec![pos], &vault, keys);
        assert_eq!(result, Some(132));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_4);
        let result = search_for_keys(&vec![pos], &vault, keys);
        assert_eq!(result, Some(136));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_5);
        let result = search_for_keys(&vec![pos], &vault, keys);
        assert_eq!(result, Some(81));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
    }

    #[test]
    fn test_search_for_keys_multi() {
        use std::time::Instant;

        const SAMPLE_INPUT_1: &str = concat!(
            "#######\n",
            "#a.#Cd#\n",
            "##...##\n",
            "##.@.##\n",
            "##...##\n",
            "#cB#Ab#\n",
            "#######\n",
        );

        const SAMPLE_INPUT_2: &str = concat!(
            "###############\n",
            "#d.ABC.#.....a#\n",
            "######...######\n",
            "######.@.######\n",
            "######...######\n",
            "#b.....#.....c#\n",
            "###############\n",
        );

        let mut test_num = 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_1);
        let (vault, positions) = replace_vault_center(&vault, pos);
        let result = search_for_keys(&positions, &vault, keys);
        assert_eq!(result, Some(8));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        // let start_time = Instant::now();
        // let (vault, pos, keys) = parse_input(SAMPLE_INPUT_2);
        // let (vault, positions) = replace_vault_center(&vault, pos);
        // let result = search_for_keys(&positions, &vault, keys);
        // assert_eq!(result, Some(24));
        // println!(
        //     "test {} ran in {} sec",
        //     test_num, start_time.elapsed().as_secs_f32()
        // );
        // test_num += 1;

        // let start_time = Instant::now();
        // let (vault, pos, keys) = parse_input(SAMPLE_INPUT_2);
        // let result = search_for_keys(&vec![pos], &vault, keys);
        // assert_eq!(result, Some(86));
        // println!(
        //     "test {} ran in {} sec",
        //     test_num, start_time.elapsed().as_secs_f32()
        // );
        // test_num += 1;

        // let start_time = Instant::now();
        // let (vault, pos, keys) = parse_input(SAMPLE_INPUT_3);
        // let result = search_for_keys(&vec![pos], &vault, keys);
        // assert_eq!(result, Some(132));
        // println!(
        //     "test {} ran in {} sec",
        //     test_num, start_time.elapsed().as_secs_f32()
        // );
        // test_num += 1;

        // let start_time = Instant::now();
        // let (vault, pos, keys) = parse_input(SAMPLE_INPUT_4);
        // let result = search_for_keys(&vec![pos], &vault, keys);
        // assert_eq!(result, Some(136));
        // println!(
        //     "test {} ran in {} sec",
        //     test_num, start_time.elapsed().as_secs_f32()
        // );
        // test_num += 1;

        // let start_time = Instant::now();
        // let (vault, pos, keys) = parse_input(SAMPLE_INPUT_5);
        // let result = search_for_keys(&vec![pos], &vault, keys);
        // assert_eq!(result, Some(81));
        // println!(
        //     "test {} ran in {} sec",
        //     test_num, start_time.elapsed().as_secs_f32()
        // );
    }
}
