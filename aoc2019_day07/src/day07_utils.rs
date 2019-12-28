use std::convert::TryInto;

use aoc2019_utils::*;
use crate::day07_cpu::*;

pub fn permute_list(list: &Vec<i64>) -> Vec<Vec<i64>> {

    if list.is_empty() {
        return vec![vec![]];
    }

    let num_results = fact(list.len().try_into().unwrap());
    let mut results = Vec::with_capacity(num_results as usize);

    for i in 0..list.len() {
        let num = list[i];
        let other_nums = {
            let mut other_nums = list.clone();
            other_nums.remove(i);
            other_nums
        };

        let others_permuted = permute_list(&other_nums);
        for other in others_permuted {
            results.push(vec![num]);
            results.last_mut().unwrap().extend_from_slice(&other[..]);
        }
    }

    results
}

fn run_prog_chain(prog: &Vec<i64>, phases: &Vec<i64>) -> i64 {
    let mut cpus: Vec<Cpu> = phases.iter()
        .map(|phase| {
            let mut cpu = Cpu::new(prog);
            cpu.set_print_output(false);
            cpu.add_input(*phase);
            cpu
        })
        .collect();

    cpus.first_mut().unwrap().add_input(0);

    let mut signal = 0;
    let num_cpus = cpus.len();
    while cpus.last().unwrap().get_state() != CpuState::Done {
        for i in 0..num_cpus {
            let cpu = &mut cpus[i];
            cpu.exec_prog();
            let outval = cpu.pop_output().unwrap();
            signal = outval;

            let next_cpu = &mut cpus[(i + 1) % num_cpus];
            next_cpu.add_input(outval);
        }
    }

    signal
}

pub fn find_max_of_all_phase_combos(prog: &Vec<i64>, phases: &Vec<i64>) -> i64 {
    let phases_list = permute_list(phases);
    phases_list.iter().map(|phases| {
        run_prog_chain(prog, phases)
    })
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute_list() {

        let result = permute_list(&vec![1, 2, 3, 4]);
        let expected = vec!(
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 3],
            vec![1, 3, 2, 4],
            vec![1, 3, 4, 2],
            vec![1, 4, 2, 3],
            vec![1, 4, 3, 2],

            vec![2, 1, 3, 4],
            vec![2, 1, 4, 3],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![2, 4, 1, 3],
            vec![2, 4, 3, 1],

            vec![3, 1, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 1, 4],
            vec![3, 2, 4, 1],
            vec![3, 4, 1, 2],
            vec![3, 4, 2, 1],

            vec![4, 1, 2, 3],
            vec![4, 1, 3, 2],
            vec![4, 2, 1, 3],
            vec![4, 2, 3, 1],
            vec![4, 3, 1, 2],
            vec![4, 3, 2, 1],
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_max_of_all_phase_combos() {
        let phases = vec![0, 1, 2, 3, 4];

        let prog_str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let prog = parse_prog(&prog_str);
        let result = find_max_of_all_phase_combos(&prog, &phases);
        assert_eq!(result, 43210);

        let prog_str = concat!(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,",
            "101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let prog = parse_prog(&prog_str);
        let result = find_max_of_all_phase_combos(&prog, &phases);
        assert_eq!(result, 54321);

        let prog_str = concat!(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,",
            "1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        let prog = parse_prog(&prog_str);
        let result = find_max_of_all_phase_combos(&prog, &phases);
        assert_eq!(result, 65210);

        let phases = vec![5, 6, 7, 8, 9];

        let prog_str = concat!(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,",
            "27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        let prog = parse_prog(&prog_str);
        let result = find_max_of_all_phase_combos(&prog, &phases);
        assert_eq!(result, 139629729);

        let prog_str = concat!(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,",
            "-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,",
            "53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        let prog = parse_prog(&prog_str);
        let result = find_max_of_all_phase_combos(&prog, &phases);
        assert_eq!(result, 18216);
    }
}
