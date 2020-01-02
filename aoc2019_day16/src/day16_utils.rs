use aoc2019_utils::*;

pub fn parse_input(input: &str) -> Vec<i32> {
    input.bytes().map(|c| (c - b'0') as i32).collect()
}

pub fn get_output_text(nums: &Vec<i32>, offset: usize, len: usize) -> String {
    nums[offset..(offset + len)].iter()
        .map(|n| (b'0' + (*n as u8)) as char)
        .collect()
}

pub fn repeat_input(nums: &Vec<i32>, num_repeats: usize) -> Vec<i32> {
    let mut output = Vec::with_capacity(nums.len() * num_repeats);
    for _ in 0..num_repeats {
        output.extend_from_slice(&nums[..]);
    }
    output
}

fn get_multiplier(in_digit_idx: usize, out_digit_idx: usize) -> i32 {
    const PATTERN: [i32; 4] = [0, 1, 0,-1];
    let idx = in_digit_idx + 1;
    let idx = (idx / (out_digit_idx + 1)) & 0b11;
    PATTERN[idx]
}

// This works and was used by calc_next_phase() for part A.
#[allow(dead_code)]
fn calc_phase_digit(input: &Vec<i32>, out_digit_idx: usize) -> i32 {
    input.iter().enumerate()
        .map(|(i, num)| num * get_multiplier(i, out_digit_idx))
        .sum::<i32>()
        .abs()
        % 10
}

// This works, is faster than calc_phase_digit(), and was used by
// calc_next_phase() for an attempt at part B, but it's still too slow.
fn calc_phase_digit_v2(input: &Vec<i32>, out_digit_idx: usize) -> i32 {
    let mut sum = 0;
    let start_idx = out_digit_idx;
    let range_len = start_idx + 1;
    let mut i = start_idx;
    while i < input.len() {
        let mut range_idx = 0;
        while (range_idx < range_len) && (i < input.len()) {
            sum += input[i] * get_multiplier(i, out_digit_idx);
            range_idx += 1;
            i += 1;
        }
        i += range_len;
    }

    sum.abs() % 10
}

// The initial calc_next_phase(). Works for calc_phase_digit() or
// calc_phase_digit_v2(). Either is ok for part A, but this is overall too
// slow for part B. See calc_next_phase_v3() for what works with part B.
pub fn calc_next_phase(input: &Vec<i32>) -> Vec<i32> {
    use std::time::Instant;

    let mut output = Vec::with_capacity(input.len());
    let start_time = Instant::now();
    let mut chunk_start_time = start_time;
    for i in 0..input.len() {
        output.push(calc_phase_digit_v2(input, i));

        if i % 1000 == 0 {
            println!("calc'd phase: {}. chunk time: {:.2}s, total time: {}",
                i,
                chunk_start_time.elapsed().as_secs_f32(),
                sec_to_hrs_mins_secs_str(start_time.elapsed().as_secs()));
            chunk_start_time = Instant::now();
        }
    }
    output
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct SumRange {
    idx: usize,
    sum: i32,
}

fn gen_initial_ranges(input: &Vec<i32>) -> Vec<SumRange> {
    input.iter().enumerate().step_by(2)
        .map(|(i, num)| SumRange { idx: i, sum: *num })
        .collect()
}

fn gen_next_ranges(ranges: &Vec<SumRange>, input: &Vec<i32>) -> Vec<SumRange> {
    if ranges.is_empty() {
        return vec![];
    }

    let mut next_ranges = Vec::with_capacity(ranges.len());

    let range_len_in = ranges[0].idx + 1;
    let range_len_out = range_len_in + 1;

    for (range_idx, range) in ranges.iter().enumerate() {
        let in_start_idx = range.idx;
        let in_end_idx = in_start_idx + range_len_in;
        let out_start_idx = in_start_idx + (2 * range_idx + 1);
        let out_end_idx = out_start_idx + range_len_out;

        let in_end_idx = std::cmp::min(in_end_idx, input.len());
        let out_start_idx = std::cmp::min(out_start_idx, input.len());
        let out_end_idx = std::cmp::min(out_end_idx, input.len());

        if out_start_idx >= input.len() {
            break;
        }

        let mut next_sum = range.sum;

        let sub_start_idx = in_start_idx;
        let sub_end_idx = std::cmp::min(in_end_idx, out_start_idx);
        for i in sub_start_idx..sub_end_idx {
            next_sum -= input[i];
        }

        let add_start_idx = std::cmp::max(in_end_idx, out_start_idx);
        let add_end_idx = out_end_idx;
        for i in add_start_idx..add_end_idx {
            next_sum += input[i];
        }

        next_ranges.push(SumRange { idx: out_start_idx, sum: next_sum });
    }

    next_ranges
}

fn calc_digit_from_ranges(ranges: &Vec<SumRange>) -> i32 {
    let to_add = ranges.iter().skip(0).step_by(2).map(|range| range.sum).sum::<i32>();
    let to_sub = ranges.iter().skip(1).step_by(2).map(|range| range.sum).sum::<i32>();
    (to_add - to_sub).abs() % 10
}

// This works for part B. It's "v3" because calc_next_phase() was really both
// v1 and v2, depending on what version of calc_phase_digit() was called, but
// v3 required a more substantial re-write.
pub fn calc_next_phase_v3(input: &Vec<i32>) -> Vec<i32> {
    let mut output = Vec::with_capacity(input.len());
    let mut ranges = gen_initial_ranges(&input);
    while !ranges.is_empty() {
        output.push(calc_digit_from_ranges(&ranges));
        ranges = gen_next_ranges(&ranges, &input);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "78463548567";
        assert_eq!(parse_input(input), vec![7, 8, 4, 6, 3, 5, 4, 8, 5, 6, 7]);
    }

    #[test]
    fn test_repeat_input() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let target = vec![
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
        ];
        let result = repeat_input(&input, 3);
        assert_eq!(result, target);
    }

    #[test]
    fn test_get_multiplier() {
        assert_eq!(get_multiplier(0, 0),  1);
        assert_eq!(get_multiplier(1, 0),  0);
        assert_eq!(get_multiplier(2, 0), -1);
        assert_eq!(get_multiplier(3, 0),  0);
        assert_eq!(get_multiplier(4, 0),  1);
        assert_eq!(get_multiplier(5, 0),  0);
        assert_eq!(get_multiplier(6, 0), -1);
        assert_eq!(get_multiplier(7, 0),  0);

        assert_eq!(get_multiplier( 0, 1),  0);
        assert_eq!(get_multiplier( 1, 1),  1);
        assert_eq!(get_multiplier( 2, 1),  1);
        assert_eq!(get_multiplier( 3, 1),  0);
        assert_eq!(get_multiplier( 4, 1),  0);
        assert_eq!(get_multiplier( 5, 1), -1);
        assert_eq!(get_multiplier( 6, 1), -1);
        assert_eq!(get_multiplier( 7, 1),  0);
        assert_eq!(get_multiplier( 8, 1),  0);
        assert_eq!(get_multiplier( 9, 1),  1);
        assert_eq!(get_multiplier(10, 1),  1);
        assert_eq!(get_multiplier(11, 1),  0);
        assert_eq!(get_multiplier(12, 1),  0);
        assert_eq!(get_multiplier(13, 1), -1);
        assert_eq!(get_multiplier(14, 1), -1);
        assert_eq!(get_multiplier(15, 1),  0);
    }

    #[test]
    fn test_calc_phase_digit() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(calc_phase_digit(&input, 0), 4);
        assert_eq!(calc_phase_digit(&input, 1), 8);
        assert_eq!(calc_phase_digit(&input, 2), 2);
        assert_eq!(calc_phase_digit(&input, 3), 2);
        assert_eq!(calc_phase_digit(&input, 4), 6);
        assert_eq!(calc_phase_digit(&input, 5), 1);
        assert_eq!(calc_phase_digit(&input, 6), 5);
        assert_eq!(calc_phase_digit(&input, 7), 8);
    }

    #[test]
    fn test_calc_next_phase() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let input = calc_next_phase(&input);
        assert_eq!(input, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        let input = calc_next_phase(&input);
        assert_eq!(input, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        let input = calc_next_phase(&input);
        assert_eq!(input, vec![0, 3, 4, 1, 5, 5, 1, 8]);

        let input = calc_next_phase(&input);
        assert_eq!(input, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_gen_initial_ranges() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let target = vec![
            SumRange { idx: 0, sum: 1 },
            SumRange { idx: 2, sum: 3 },
            SumRange { idx: 4, sum: 5 },
            SumRange { idx: 6, sum: 7 },
        ];
        let result = gen_initial_ranges(&input);
        assert_eq!(target, result);
    }

    #[test]
    fn test_gen_next_ranges() {
        let input = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        ];

        let target = vec![
            SumRange { idx: 0, sum: 1 },
            SumRange { idx: 2, sum: 3 },
            SumRange { idx: 4, sum: 5 },
            SumRange { idx: 6, sum: 7 },
            SumRange { idx: 8, sum: 9 },
            SumRange { idx: 10, sum: 1 },
            SumRange { idx: 12, sum: 3 },
            SumRange { idx: 14, sum: 5 },
            SumRange { idx: 16, sum: 7 },
            SumRange { idx: 18, sum: 9 },
        ];
        let ranges = gen_initial_ranges(&input);
        assert_eq!(ranges, target);

        let target = vec![
            SumRange { idx: 1, sum: 5 },
            SumRange { idx: 5, sum: 13 },
            SumRange { idx: 9, sum: 1 },
            SumRange { idx: 13, sum: 9 },
            SumRange { idx: 17, sum: 17 },
        ];
        let ranges = gen_next_ranges(&ranges, &input);
        assert_eq!(ranges, target);

        let target = vec![
            SumRange { idx: 2, sum: 12 },
            SumRange { idx: 8, sum: 10 },
            SumRange { idx: 14, sum: 18 },
        ];
        let ranges = gen_next_ranges(&ranges, &input);
        assert_eq!(ranges, target);

        let target = vec![
            SumRange { idx: 3, sum: 22 },
            SumRange { idx: 11, sum: 14 },
            SumRange { idx: 19, sum: 0 },
        ];
        let ranges = gen_next_ranges(&ranges, &input);
        assert_eq!(ranges, target);

        let target = vec![
            SumRange { idx: 4, sum: 35 },
            SumRange { idx: 14, sum: 35 },
        ];
        let ranges = gen_next_ranges(&ranges, &input);
        assert_eq!(ranges, target);
    }

    #[test]
    fn test_calc_digit_from_ranges() {
        let ranges = vec![
            SumRange { idx: 0, sum: 1 },
            SumRange { idx: 2, sum: 3 },
            SumRange { idx: 4, sum: 5 },
            SumRange { idx: 6, sum: 7 },
        ];
        let result = calc_digit_from_ranges(&ranges);
        assert_eq!(result, 4);

        let ranges = vec![
            SumRange { idx: 0, sum: 1 },
            SumRange { idx: 2, sum: -3 },
            SumRange { idx: 4, sum: 5 },
            SumRange { idx: 6, sum: -7 },
        ];
        let result = calc_digit_from_ranges(&ranges);
        assert_eq!(result, 6);
    }
}
