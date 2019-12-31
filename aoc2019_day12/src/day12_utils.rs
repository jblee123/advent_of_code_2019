use std::str::FromStr;

use aoc2019_utils::*;

pub type MoonPos = vec3d::Vec3d<i16>;
pub type MoonVel = vec3d::Vec3d<i16>;

const ZERO_VEL: MoonVel = MoonVel { x: 0, y: 0, z: 0 };

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Moon {
    pub pos: MoonPos,
    pub vel: MoonVel,
}

impl Moon {
    pub fn from_nums(
        pos_x: i16,
        pos_y: i16,
        pos_z: i16,
        vel_x: i16,
        vel_y: i16,
        vel_z: i16,
    ) -> Self {
        Moon {
            pos: MoonPos { x: pos_x, y: pos_y, z: pos_z },
            vel: MoonVel { x: vel_x, y: vel_y, z: vel_z },
        }
    }

    pub fn apply_gravity(&mut self, other_pos: MoonPos) {
        let mut to_other = other_pos - self.pos;
        to_other.x /= if to_other.x != 0 { to_other.x.abs() } else { 1 };
        to_other.y /= if to_other.y != 0 { to_other.y.abs() } else { 1 };
        to_other.z /= if to_other.z != 0 { to_other.z.abs() } else { 1 };
        self.vel += to_other;
    }

    pub fn apply_vel(&mut self) {
        self.pos += self.vel;
    }

    pub fn get_potential(&self) -> i32 {
        self.pos.x.abs() as i32
            + self.pos.y.abs() as i32
            + self.pos.z.abs() as i32
    }

    pub fn get_kinetic(&self) -> i32 {
        self.vel.x.abs() as i32
            + self.vel.y.abs() as i32
            + self.vel.z.abs() as i32
    }

    pub fn get_energy(&self) -> i32 {
        self.get_potential() * self.get_kinetic()
    }
}

pub fn parse_input(input: &str) -> Vec<MoonPos> {
    input.lines().map(|line| {
        let mut parts = line[1..(line.len() - 1)].split(", ");
        let x_str = parts.next().unwrap();
        let y_str = parts.next().unwrap();
        let z_str = parts.next().unwrap();
        MoonPos {
            x: i16::from_str(&x_str[2..]).unwrap(),
            y: i16::from_str(&y_str[2..]).unwrap(),
            z: i16::from_str(&z_str[2..]).unwrap(),
        }
    }).collect()
}

pub fn create_moons(positions: &Vec<MoonPos>) -> Vec<Moon> {
    positions.iter().map(|pos| {
        Moon {
            pos: *pos,
            vel: ZERO_VEL,
        }
    }).collect()
}

pub fn sim_time_step(moons: &mut Vec<Moon>) {
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

pub fn get_system_energy(moons: &Vec<Moon>) -> i32 {
    moons.iter().map(|moon| moon.get_energy()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO_POS: MoonPos = MoonPos { x: 0, y: 0, z: 0 };

    const SIMPLE_TEST_INPUT: &str = concat!(
        "<x=-1, y=0, z=2>\n",
        "<x=2, y=-10, z=-7>\n",
        "<x=4, y=-8, z=8>\n",
        "<x=3, y=5, z=-1>\n",
    );

    #[test]
    fn test_moon_apply_gravity() {
        let mut moon = Moon { pos: ZERO_POS, vel: ZERO_VEL, };
        moon.apply_gravity(ZERO_POS);
        assert_eq!(moon.vel, MoonVel { x: 0, y: 0, z: 0 });

        let mut moon = Moon { pos: ZERO_POS, vel: ZERO_VEL, };
        moon.apply_gravity(MoonPos { x: 20, y: 30, z: 40 });
        assert_eq!(moon.vel, MoonVel { x: 1, y: 1, z: 1 });

        let mut moon = Moon { pos: ZERO_POS, vel: ZERO_VEL, };
        moon.apply_gravity(MoonPos { x: -20, y: -30, z: -40 });
        assert_eq!(moon.vel, MoonVel { x: -1, y: -1, z: -1 });

        let mut moon = Moon { pos: ZERO_POS, vel: ZERO_VEL, };
        moon.apply_gravity(MoonPos { x: 20, y: 30, z: -40 });
        assert_eq!(moon.vel, MoonVel { x: 1, y: 1, z: -1 });

        let mut moon = Moon { pos: ZERO_POS, vel: ZERO_VEL, };
        moon.apply_gravity(MoonPos { x: 20, y: -30, z: 40 });
        assert_eq!(moon.vel, MoonVel { x: 1, y: -1, z: 1 });

        let mut moon = Moon { pos: ZERO_POS, vel: ZERO_VEL, };
        moon.apply_gravity(MoonPos { x: -20, y: 30, z: 40 });
        assert_eq!(moon.vel, MoonVel { x: -1, y: 1, z: 1 });
    }

    #[test]
    fn test_moon_apply_vel() {
        let mut moon = Moon { pos: ZERO_POS, vel: ZERO_VEL, };
        moon.apply_vel();
        assert_eq!(moon.pos, MoonPos { x: 0, y: 0, z: 0 });

        let mut moon = Moon {
            pos: MoonPos { x: 1, y: 2, z: 3 },
            vel: MoonVel { x: 10, y: 20, z: 30 },
        };
        moon.apply_vel();
        assert_eq!(moon.pos, MoonPos { x: 11, y: 22, z: 33 });
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(SIMPLE_TEST_INPUT);
        assert_eq!(result, vec![
            MoonPos { x: -1, y: 0, z: 2 },
            MoonPos { x: 2, y: -10, z: -7 },
            MoonPos { x: 4, y: -8, z: 8 },
            MoonPos { x: 3, y: 5, z: -1 },
        ]);
    }

    #[test]
    fn test_create_moons() {
        let result = parse_input(SIMPLE_TEST_INPUT);
        let result = create_moons(&result);
        assert_eq!(result, vec![
            Moon { pos: MoonPos { x: -1, y: 0, z: 2 }, vel: ZERO_VEL },
            Moon { pos: MoonPos { x: 2, y: -10, z: -7 }, vel: ZERO_VEL },
            Moon { pos: MoonPos { x: 4, y: -8, z: 8 }, vel: ZERO_VEL },
            Moon { pos: MoonPos { x: 3, y: 5, z: -1 }, vel: ZERO_VEL },
        ]);
    }

    #[test]
    fn test_sim_time_step() {
        let input = parse_input(SIMPLE_TEST_INPUT);
        let mut moons = create_moons(&input);

        sim_time_step(&mut moons);
        assert_eq!(moons, vec![
            Moon::from_nums(2, -1,  1,  3, -1, -1),
            Moon::from_nums(3, -7, -4,  1,  3,  3),
            Moon::from_nums(1, -7,  5, -3,  1, -3),
            Moon::from_nums(2,  2,  0, -1, -3,  1),
        ]);

        sim_time_step(&mut moons);
        assert_eq!(moons, vec![
            Moon::from_nums(5, -3, -1,  3, -2, -2),
            Moon::from_nums(1, -2,  2, -2,  5,  6),
            Moon::from_nums(1, -4, -1,  0,  3, -6),
            Moon::from_nums(1, -4,  2, -1, -6,  2),
        ]);
    }

    #[test]
    fn test_moon_get_energy() {
        let result = Moon::from_nums(2,  1, -3, -3, -2,  1).get_energy();
        assert_eq!(result, 36);

        let result = Moon::from_nums(1, -8,  0, -1,  1,  3).get_energy();
        assert_eq!(result, 45);

        let result = Moon::from_nums(3, -6,  1,  3,  2, -3).get_energy();
        assert_eq!(result, 80);

        let result = Moon::from_nums(2,  0,  4,  1, -1, -1).get_energy();
        assert_eq!(result, 18);
    }

    #[test]
    fn test_get_system_energy() {
        let moons = vec![
            Moon::from_nums(2,  1, -3, -3, -2,  1),
            Moon::from_nums(1, -8,  0, -1,  1,  3),
            Moon::from_nums(3, -6,  1,  3,  2, -3),
            Moon::from_nums(2,  0,  4,  1, -1, -1),
        ];

        let result = get_system_energy(&moons);
        assert_eq!(result, 179);
    }
}
