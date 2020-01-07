use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

use aoc2019_utils::*;

const WALL: char = '#';
const SPACE: char = '.';

pub type Coord = point_2d::Point2d<u8>;
pub type KeySet = u32;
pub type KeyMask = u32;

const ZERO_COORD: Coord = Coord { x: 0, y: 0 };
const NUM_LETTERS: usize = 26;

pub type Vault = Vec<Vec<char>>;

fn vault_cell_at(vault: &Vault, coord: Coord) -> char {
    vault[coord.x as usize][coord.y as usize]
}

fn set_vault_cell_at(vault: &mut Vault, coord: Coord, c: char) {
    vault[coord.x as usize][coord.y as usize] = c;
}

fn key_letter_to_bit(c: char) -> u32 {
    assert!(('a'..='z').contains(&c));
    1 << (c as u8 - b'a') as u32
}

fn lock_letter_to_bit(c: char) -> u32 {
    assert!(('A'..='Z').contains(&c));
    1 << (c as u8 - b'A') as u32
}

fn get_num_keys(keys: KeySet) -> u32 {
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

pub fn parse_input(input: &str) -> (Vault, Coord) {
    let lines = input.lines().collect::<Vec<&str>>();

    let height = lines.len();
    let width = lines[0].len();

    let mut pos = ZERO_COORD;

    let mut vault = vec![vec![SPACE; height]; width];
    for x in 0..width {
        for y in 0..height {
            let c = lines[y].as_bytes()[x] as char;
            vault[x][y] = c;
            match c {
                '@' => pos = Coord { x: x as u8, y: y as u8 },
                _ => {},
            }
        }
    }

    (vault, pos)
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

fn is_door(cell: char) -> bool {
    match cell {
        'A'..='Z' => true,
        _ => false,
    }
}

fn is_key(cell: char) -> bool {
    match cell {
        'a'..='z' => true,
        _ => false,
    }
}

fn is_wall(cell: char) -> bool {
    cell == WALL
}

fn calc_map_stats_single(vault: &Vault, start: Coord)
-> (KeySet, Vec<Coord>, Vec<u32>) {

    struct Visitation { pos: Coord, passed_doors: KeySet }

    let width = vault.len();
    let height = vault[0].len();

    let mut history = HashSet::new();
    let mut to_visit = LinkedList::new();

    let mut valid_keys = 0;
    let mut key_coords = vec![ZERO_COORD; NUM_LETTERS];
    let mut needed_key_map = vec![0; NUM_LETTERS];

    to_visit.push_back(Visitation { pos: start, passed_doors: 0 });
    while !to_visit.is_empty() {

        let visitation = to_visit.pop_front().unwrap();
        history.insert(visitation.pos.clone());

        let c = vault_cell_at(&vault, visitation.pos);

        let passed_doors = if is_door(c) {
            visitation.passed_doors | lock_letter_to_bit(c)
        } else {
            visitation.passed_doors
        };

        if is_key(c) {
            let key_idx = (c as u8 - b'a') as usize;
            valid_keys |= key_letter_to_bit(c);
            key_coords[key_idx] = visitation.pos;
            needed_key_map[key_idx] = passed_doors;
        }

        let reachables = get_reachable_coords(visitation.pos, width, height);
        let next_visits = reachables.iter()
            .filter(|coord| {
                !is_wall(vault_cell_at(&vault, **coord))
                    && !history.contains(coord)
            });

        for coord in next_visits {
            to_visit.push_back(
                Visitation {
                    pos: *coord,
                    passed_doors: passed_doors,
                }
            );
        }
    }

    (valid_keys, key_coords, needed_key_map)
}


fn calc_map_stats(vault: &Vault, starts: &Vec<Coord>)
-> (KeySet, Vec<Coord>, Vec<u32>, Vec<Option<usize>>) {
    let mut valid_keys = 0;
    let mut key_coords = vec![ZERO_COORD; NUM_LETTERS];
    let mut needed_key_map = vec![0; NUM_LETTERS];
    let mut key_start_idx_map = vec![None; NUM_LETTERS];

    for (start_idx, start) in starts.iter().enumerate() {
        let (valid_keys2, key_coords2, needed_key_map2) =
            calc_map_stats_single(&vault, *start);

        valid_keys |= valid_keys2;

        for (i, coord) in key_coords2.iter().enumerate() {
            if *coord != ZERO_COORD {
                key_coords[i] = *coord;
            }
        }

        for (i, coord) in needed_key_map2.iter().enumerate() {
            needed_key_map[i] |= *coord;
        }

        for i in 0..NUM_LETTERS {
            let is_valid_key = ((1 << i) & valid_keys2) != 0;
            if is_valid_key {
                key_start_idx_map[i] = Some(start_idx);
            }
        }
    }

    (valid_keys, key_coords, needed_key_map, key_start_idx_map)
}

fn calc_dist_between(vault: &Vault, p1: Coord, p2: Coord) -> Option<u32> {

    struct Visitation { pos: Coord, dist: u32 }

    let width = vault.len();
    let height = vault[0].len();

    let mut history = HashSet::new();
    let mut to_visit = LinkedList::new();

    to_visit.push_back(Visitation { pos: p1, dist: 0 });
    while !to_visit.is_empty() {

        let visitation = to_visit.pop_front().unwrap();
        history.insert(visitation.pos.clone());

        if visitation.pos == p2 {
            return Some(visitation.dist);
        }

        let reachables = get_reachable_coords(visitation.pos, width, height);
        let next_visits = reachables.iter()
            .filter(|coord| {
                !is_wall(vault_cell_at(&vault, **coord))
                    && !history.contains(coord)
            });

        for coord in next_visits {
            to_visit.push_back(
                Visitation {
                    pos: *coord,
                    dist: visitation.dist + 1,
                }
            );
        }
    }

    None
}

fn key_to_idx(key: char) -> usize {
    assert!(('a'..='z').contains(&key));
    (key as u8 - b'a') as usize
}

fn get_key_distances_map_idx(key1: char, key2: char) -> usize {
    key_to_idx(key1) * NUM_LETTERS + key_to_idx(key2)
}

fn is_valid_key(key: char, valid_keys: KeySet) -> bool {
    (key_letter_to_bit(key) & valid_keys) != 0
}

fn calc_dists_btwn_keys(
    vault: &Vault,
    key_coords: &Vec<Coord>,
    valid_keys: KeySet,
) -> Vec<Option<u32>> {

    let mut dist_map = vec![None; NUM_LETTERS * NUM_LETTERS];

    let valid_keys = (b'a'..=b'z')
        .filter(|key| is_valid_key(*key as char, valid_keys))
        .map(|key| key as char)
        .collect::<Vec<char>>();

    for &key1 in valid_keys.iter() {
        for &key2 in valid_keys.iter() {
            if key1 < key2 {
                let p1 = key_coords[key_to_idx(key1)];
                let p2 = key_coords[key_to_idx(key2)];
                let dist = calc_dist_between(&vault, p1, p2);
                let map_idx1 = get_key_distances_map_idx(key1, key2);
                let map_idx2 = get_key_distances_map_idx(key2, key1);
                dist_map[map_idx1] = dist;
                dist_map[map_idx2] = dist;
            }
        }
    }

    dist_map
}

fn calc_key_start_dists(
    vault: &Vault,
    starts: &Vec<Coord>,
    key_coords: &Vec<Coord>,
    valid_keys: KeySet,
) -> Vec<Option<u32>> {

    let mut dist_map = vec![None; NUM_LETTERS];

    let valid_keys = (b'a'..=b'z')
        .filter(|key| is_valid_key(*key as char, valid_keys))
        .map(|key| key as char)
        .collect::<Vec<char>>();

    for &key in valid_keys.iter() {
        for &start in starts.iter() {
            let key_pos = key_coords[key_to_idx(key)];
            let dist = calc_dist_between(&vault, key_pos, start);
            if dist.is_some() {
                dist_map[key_to_idx(key)] = dist;
            }
        }
    }

    dist_map
}

fn get_next_keys_to_try(
    keys_remaining: KeySet,
    held_keys: KeySet,
    needed_key_map: &Vec<u32>
)
-> Vec<char> {
    let mut next_keys = vec![];

    for c in b'a'..=b'z' {
        let c = c as char;
        let mask = key_letter_to_bit(c);
        let required = needed_key_map[key_to_idx(c)];

        let key_remains = mask & keys_remaining != 0;
        let have_required_keys = (required & held_keys) == required;

        if key_remains && have_required_keys {
            next_keys.push(c);
        }
    }

    next_keys
}

pub fn search_for_keys(vault: &Vault, starts: &Vec<Coord>) -> Option<u32> {

    let (valid_keys, key_coords, needed_key_map, key_start_idx_map) =
        calc_map_stats(&vault, starts);
    let key_distances = calc_dists_btwn_keys(&vault, &key_coords, valid_keys);
    let key_start_distances = calc_key_start_dists(
        &vault,
        starts,
        &key_coords,
        valid_keys,
    );

    #[derive(Clone, Debug)]
    struct KeyChain {
        chain_letters: Vec<char>,
        num_steps: u32,
        keys_collected: KeySet,
        keys_remaining: KeySet,
        last_key: Option<u8>,
        last_keys_per_start: Vec<Option<u8>>,
        locs: Vec<Coord>,
    }

    let mut valid_chains = vec![
        KeyChain {
            chain_letters: vec![],
            num_steps: 0,
            keys_collected: 0,
            keys_remaining: valid_keys,
            last_key: None,
            last_keys_per_start: vec![None; starts.len()],
            locs: starts.clone(),
        }
    ];

    let num_valid_keys = get_num_keys(valid_keys);

    for _ in 0..num_valid_keys {
        if valid_chains.is_empty() {
            break;
        }

        #[derive(Clone, PartialEq, Eq, Hash)]
        struct KeyChainHashKey {
            keys_collected: u32,
            last_key: char,
            locs: Vec<Coord>,
        }
        let mut next_chains = HashMap::<KeyChainHashKey, KeyChain>::new();

        for chain in &valid_chains {
            let next_keys = get_next_keys_to_try(
                chain.keys_remaining,
                chain.keys_collected,
                &needed_key_map
            );

            for next_key in next_keys {
                let next_key_idx = key_to_idx(next_key);
                let next_key_start_idx =
                    key_start_idx_map[next_key_idx].unwrap();
                let dist = if let Some(prev_key_for_dist) =
                    chain.last_keys_per_start[next_key_start_idx]
                {
                    let dist_map_idx = get_key_distances_map_idx(
                        prev_key_for_dist as char,
                        next_key);
                    key_distances[dist_map_idx]
                } else {
                    key_start_distances[next_key_idx]
                };

                if !dist.is_some() {
                    continue;
                }

                let next_key_mask = key_letter_to_bit(next_key);
                let next_keys_collected = chain.keys_collected | next_key_mask;
                let next_keys_remaining = chain.keys_remaining & !next_key_mask;
                let next_dist = chain.num_steps + dist.unwrap();
                let mut next_last_keys_per_start = chain.last_keys_per_start.clone();
                next_last_keys_per_start[next_key_start_idx] = Some(next_key as u8);
                let mut next_chain_letters = chain.chain_letters.clone();
                next_chain_letters.push(next_key);
                let mut next_locs = chain.locs.clone();
                next_locs[next_key_start_idx] = key_coords[next_key_idx];
                let next_chain_key = KeyChainHashKey {
                    keys_collected: next_keys_collected,
                    last_key: next_key,
                    locs: next_locs.clone(),
                };

                let next_chain = KeyChain {
                    num_steps: next_dist,
                    keys_collected: next_keys_collected,
                    keys_remaining: next_keys_remaining,
                    last_key: Some(next_key as u8),
                    last_keys_per_start: next_last_keys_per_start,
                    chain_letters: next_chain_letters,
                    locs: next_locs,
                };

                if let Some(so_far) = next_chains.get_mut(&next_chain_key) {
                    if so_far.num_steps > next_dist {
                        *so_far = next_chain;
                    }
                } else {
                    next_chains.insert(
                        next_chain_key,
                        next_chain,
                    );
                }
            }
        }

        valid_chains = next_chains.values().cloned().collect();
    }

    if valid_chains.is_empty() {
        return None;
    }

    let min_steps = valid_chains.iter()
        .map(|chain| chain.num_steps)
        .min()
        .unwrap();

    Some(min_steps)
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
        let (vault, pos) = parse_input(SAMPLE_INPUT_1);
        assert_eq!(vault, target_vault);
        assert_eq!(pos, target_pos);
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

        let (target, _) = parse_input(MAP_POST);
        let (result, pos) = parse_input(MAP_PRE);
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
        let (vault, pos) = parse_input(SAMPLE_INPUT_1);
        let result = search_for_keys(&vault, &vec![pos]);
        assert_eq!(result, Some(8));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos) = parse_input(SAMPLE_INPUT_2);
        let result = search_for_keys(&vault, &vec![pos]);
        assert_eq!(result, Some(86));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos) = parse_input(SAMPLE_INPUT_3);
        let result = search_for_keys(&vault, &vec![pos]);
        assert_eq!(result, Some(132));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos) = parse_input(SAMPLE_INPUT_4);
        let result = search_for_keys(&vault, &vec![pos]);
        assert_eq!(result, Some(136));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos) = parse_input(SAMPLE_INPUT_5);
        let result = search_for_keys(&vault, &vec![pos]);
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

        const SAMPLE_INPUT_3: &str = concat!(
            "#############\n",
            "#DcBa.#.GhKl#\n",
            "#.###...#I###\n",
            "#e#d#.@.#j#k#\n",
            "###C#...###J#\n",
            "#fEbA.#.FgHi#\n",
            "#############\n",
        );

        const SAMPLE_INPUT_4: &str = concat!(
            "#############\n",
            "#g#f.D#..h#l#\n",
            "#F###e#E###.#\n",
            "#dCba...BcIJ#\n",
            "#####.@.#####\n",
            "#nK.L...G...#\n",
            "#M###N#H###.#\n",
            "#o#m..#i#jk.#\n",
            "#############\n",
        );

        let mut test_num = 1;

        let start_time = Instant::now();
        let (vault, pos) = parse_input(SAMPLE_INPUT_1);
        let (vault, positions) = replace_vault_center(&vault, pos);
        let result = search_for_keys(&vault, &positions);
        assert_eq!(result, Some(8));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos) = parse_input(SAMPLE_INPUT_2);
        let (vault, positions) = replace_vault_center(&vault, pos);
        let result = search_for_keys(&vault, &positions);
        assert_eq!(result, Some(24));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos) = parse_input(SAMPLE_INPUT_3);
        let (vault, positions) = replace_vault_center(&vault, pos);
        let result = search_for_keys(&vault, &positions);
        assert_eq!(result, Some(32));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos) = parse_input(SAMPLE_INPUT_4);
        let (vault, positions) = replace_vault_center(&vault, pos);
        let result = search_for_keys(&vault, &positions);
        assert_eq!(result, Some(72));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
    }



    #[test]
    fn test_get_next_keys_to_try() {
        let mut needed_key_map = vec![0; NUM_LETTERS];
        needed_key_map[0] = 0b00000;
        needed_key_map[1] = 0b00000;
        needed_key_map[2] = 0b00001;
        needed_key_map[3] = 0b00000;
        needed_key_map[4] = 0b00100;
        let target = vec!['b', 'c'];
        let result = get_next_keys_to_try(0b10110, 0b00001, &needed_key_map);
        assert_eq!(result, target);
    }

    #[test]
    fn test_calc_map_stats() {
        const SAMPLE_INPUT: &str = concat!(
            "############\n",
            "#...a...B..#\n",
            "##########.#\n",
            "#...b.DC..@#\n",
            "##########.#\n",
            "#...c......#\n",
            "##########.#\n",
            "#..........#\n",
            "############\n",
        );

        let (vault, _) = parse_input(SAMPLE_INPUT);

        let coord = Coord { x: 10, y: 3 };
        let (valid_keys, key_coords, needed_key_map, key_start_idx_map) =
            calc_map_stats(&vault, &vec![coord]);

        assert_eq!(valid_keys, 0b111);

        let mut target_key_coords = vec![ZERO_COORD; NUM_LETTERS];
        target_key_coords[0] = Coord { x: 4, y: 1 };
        target_key_coords[1] = Coord { x: 4, y: 3 };
        target_key_coords[2] = Coord { x: 4, y: 5 };
        assert_eq!(key_coords, target_key_coords);

        let mut target_neede_key_map = vec![0; NUM_LETTERS];
        target_neede_key_map[0] = 0b10;
        target_neede_key_map[1] = 0b1100;
        assert_eq!(needed_key_map, target_neede_key_map);

        let mut target_key_start_idx_map = vec![None; NUM_LETTERS];
        target_key_start_idx_map[0] = Some(0);
        target_key_start_idx_map[1] = Some(0);
        target_key_start_idx_map[2] = Some(0);
        assert_eq!(key_start_idx_map, target_key_start_idx_map);
    }

    #[test]
    fn test_calc_dist_between() {
        const SAMPLE_INPUT: &str = concat!(
            "############\n",
            "#...a...B..#\n",
            "##########.#\n",
            "#...b.DC..@#\n",
            "##########.#\n",
            "#...c......#\n",
            "##########.#\n",
            "#..........#\n",
            "############\n",
        );

        let (vault, _) = parse_input(SAMPLE_INPUT);

        let p1 = Coord { x: 1, y: 1 };
        let p2 = Coord { x: 1, y: 3 };
        let result = calc_dist_between(&vault, p1, p2);
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_calc_key_distances() {
        const SAMPLE_INPUT: &str = concat!(
            "############\n",
            "#...a...B..#\n",
            "##########.#\n",
            "#...b.DC..@#\n",
            "##########.#\n",
            "#....c.....#\n",
            "##########.#\n",
            "#..........#\n",
            "############\n",
        );

        let (vault, _) = parse_input(SAMPLE_INPUT);

        let mut key_coords = vec![ZERO_COORD; NUM_LETTERS];
        key_coords[0] = Coord { x: 4, y: 1 };
        key_coords[1] = Coord { x: 4, y: 3 };
        key_coords[2] = Coord { x: 5, y: 5 };

        let mut target = vec![None; NUM_LETTERS * NUM_LETTERS];
        target[1] = Some(14);
        target[26] = Some(14);

        target[2] = Some(15);
        target[52] = Some(15);

        target[28] = Some(13);
        target[53] = Some(13);

        let result = calc_dists_btwn_keys(&vault, &key_coords, 0b111);
        assert_eq!(result, target);
    }
}
