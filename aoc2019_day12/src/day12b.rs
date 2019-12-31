pub mod day12_utils;

use aoc2019_utils::*;
use day12_utils::*;

enum VecDim { X, Y, Z }

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Moon1d {
    pos: i16,
    vel: i16,
}

impl Moon1d {
    fn apply_gravity(&mut self, other_pos: i16) {
        let mut to_other = other_pos - self.pos;
        to_other /= if to_other != 0 { to_other.abs() } else { 1 };
        self.vel += to_other;
    }

    fn apply_vel(&mut self) {
        self.pos += self.vel;
    }
}

fn create_moons_1d(positions: &Vec<MoonPos>, dim: VecDim) -> Vec<Moon1d> {
    positions.iter().map(|pos| {
        Moon1d {
            pos: match dim {
                VecDim::X => pos.x,
                VecDim::Y => pos.y,
                VecDim::Z => pos.z,
            },
            vel: 0,
        }
    }).collect()
}

fn sim_time_step_1d(moons: &mut Vec<Moon1d>) {
    (0..moons.len()).for_each(|i| {
        (0..moons.len()).for_each(|j| {
            let moon_j_pos = { moons[j].pos };
            moons[i].apply_gravity(moon_j_pos);
        });
    });

    for moon in moons {
        moon.apply_vel();
    }
}

fn get_repeat_of_origin(moon_positions: &Vec<MoonPos>, dim: VecDim) -> u64 {
    let mut moons = create_moons_1d(&moon_positions, dim);
    let start_moons = moons.clone();

    let mut step_count = 1u64;

    loop {
        sim_time_step_1d(&mut moons);

        if moons == start_moons {
            break;
        }

        step_count += 1;
    }

    step_count
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let input = aoc2019_utils::get_input("inputs/day12.txt");
    let moon_positions = parse_input(&input);
    let num_steps_x = get_repeat_of_origin(&moon_positions, VecDim::X);
    let num_steps_y = get_repeat_of_origin(&moon_positions, VecDim::Y);
    let num_steps_z = get_repeat_of_origin(&moon_positions, VecDim::Z);
    println!("first repeats for x/y/z: {} / {} / {}",
        num_steps_x, num_steps_y, num_steps_z);

    let ans = lcm(lcm(num_steps_x, num_steps_y), num_steps_z);
    println!("first repeat after {} steps", ans);
}
