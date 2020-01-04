use std::collections::HashSet;
use std::collections::LinkedList;
use std::time::Instant;

use aoc2019_utils::*;

const WALL: char = '#';

pub type Coord = point_2d::Point2d<u8>;

pub type Vault = Vec<Vec<char>>;

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

pub fn parse_input(input: &str) -> (Vault, Coord, u32) {
    let lines = input.lines().collect::<Vec<&str>>();

    let height = lines.len();
    let width = lines[0].len();

    let mut pos = Coord { x: 0, y: 0 };
    let mut keys = 0;

    let mut vault = vec![vec!['.'; height]; width];
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct KeySearchVisitation {
    pos: Coord,
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
    history: &HashSet<KeySearchVisitation>
) -> Vec<KeySearchState> {
    let width = vault.len();
    let height = vault[0].len();
    let cur_pos = start_state.visitation.pos;
    let cur_keys = start_state.visitation.keys;

    let next_coords = {
        let mut next_coords = Vec::with_capacity(4);
        if cur_pos.y > 0 {
            next_coords.push(Coord { x: cur_pos.x, y: cur_pos.y - 1 });
        }
        if cur_pos.y < height as u8 - 1 {
            next_coords.push(Coord { x: cur_pos.x, y: cur_pos.y + 1 });
        }
        if cur_pos.x > 0 {
            next_coords.push(Coord { x: cur_pos.x - 1, y: cur_pos.y });
        }
        if cur_pos.x < width as u8 - 1 {
            next_coords.push(Coord { x: cur_pos.x + 1, y: cur_pos.y });
        }

        next_coords
    };

    let mut next_states = Vec::with_capacity(4);

    for next_coord in next_coords {
        let c = vault[next_coord.x as usize][next_coord.y as usize];

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

        let new_visitation = KeySearchVisitation {
            pos: next_coord,
            keys: new_keys,
        };

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
    start_pos: Coord,
    vault: &Vault,
    all_keys: u32,
) -> Option<u32> {

    let mut to_visit = LinkedList::new();
    let mut history = HashSet::new();

    let first_state =  KeySearchState {
        visitation: KeySearchVisitation { pos: start_pos, keys: 0 },
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
            vault,
            &history
        );
        to_visit.extend(next_states.into_iter());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_parse_input() {
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
    fn test_search_for_keys() {
        use std::time::Instant;

        let mut test_num = 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_1);
        let result = search_for_keys(pos, &vault, keys);
        assert_eq!(result, Some(8));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_2);
        let result = search_for_keys(pos, &vault, keys);
        assert_eq!(result, Some(86));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_3);
        let result = search_for_keys(pos, &vault, keys);
        assert_eq!(result, Some(132));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_4);
        let result = search_for_keys(pos, &vault, keys);
        assert_eq!(result, Some(136));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
        test_num += 1;

        let start_time = Instant::now();
        let (vault, pos, keys) = parse_input(SAMPLE_INPUT_5);
        let result = search_for_keys(pos, &vault, keys);
        assert_eq!(result, Some(81));
        println!(
            "test {} ran in {} sec",
            test_num, start_time.elapsed().as_secs_f32()
        );
    }
}
