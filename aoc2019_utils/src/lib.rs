pub mod point_2d;

use std::fs;

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

pub fn get_input(filename: &str) -> String {
    let err_msg = format!("Something went wrong reading the input file: {}",
        filename);
    fs::read_to_string(filename).expect(&err_msg)
}

pub fn fact(n: u64) -> u64 {
    let mut n = n;
    let mut ans = 1;
    while n > 0 {
        ans *= n;
        n -= 1;
    }
    ans
}

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn angle_to_0_2pi(ang_rad: f32) -> f32 {
    let mut ang_rad = ang_rad;
    while ang_rad > TWO_PI {
        ang_rad -= TWO_PI;
    }
    while ang_rad < 0.0 {
        ang_rad += TWO_PI;
    }
    ang_rad
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(6), 720);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 2), 2);
        assert_eq!(gcd(2, 0), 2);
        assert_eq!(gcd(1, 12), 1);
        assert_eq!(gcd(10, 5), 5);
        assert_eq!(gcd(10, 8), 2);
        assert_eq!(gcd(8, 10), 2);
        assert_eq!(gcd(13, 14), 1);
    }

    #[test]
    fn test_angle_to_0_2pi() {
        const EPS: f32 = 0.00001;
        let val = std::f32::consts::PI;
        assert!((angle_to_0_2pi(val) - val).abs() < EPS);
        assert!((angle_to_0_2pi(val + TWO_PI) - val).abs() < EPS);
        assert!((angle_to_0_2pi(val - TWO_PI) - val).abs() < EPS);
    }
}
