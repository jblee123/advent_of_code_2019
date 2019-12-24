// pub mod aoc2019_day1_utils;

use std::str::FromStr;

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc_fuel() {
        use super::*;

        assert_eq!(calc_fuel(0), 0);
        assert_eq!(calc_fuel(3), 0);
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);
    }

    #[test]
    fn test_calc_fuel_from_str() {
        use super::*;

        let input = concat!(
            "0\n",
            "3\n",
            "12\n",
            "14\n",
            "1969\n",
            "100756\n",
        );

        assert_eq!(calc_fuel_from_str(&input), 34241);
    }

    #[test]
    fn test_calc_fuel_for_fuel() {
        use super::*;

        assert_eq!(calc_fuel_for_fuel(2), 2);
        assert_eq!(calc_fuel_for_fuel(654), 966);
        assert_eq!(calc_fuel_for_fuel(33583), 50346);
    }

    #[test]
    fn test_calc_fuel_for_fuel_from_str() {
        use super::*;

        let input = concat!(
            "14\n",
            "1969\n",
            "100756\n",
        );

        assert_eq!(calc_fuel_for_fuel_from_str(&input), 51314);
    }
}

fn calc_fuel(mass: i32) -> i32 {
    std::cmp::max(mass / 3 - 2, 0)
}

fn calc_fuel_for_fuel(fuel: i32) -> i32 {
    let mut extra_fuel = fuel;
    let mut total_fuel = fuel;

    while extra_fuel > 0 {
        extra_fuel = calc_fuel(extra_fuel);
        total_fuel += extra_fuel;
    }

    total_fuel
}

pub fn calc_fuel_from_str(s: &str) -> i32 {
    s.lines().map(|line| {
        calc_fuel(i32::from_str(line).unwrap())
    }).sum::<i32>()
}

pub fn calc_fuel_for_fuel_from_str(s: &str) -> i32 {
    s.lines().map(|line| {
        let fuel = calc_fuel(i32::from_str(line).unwrap());
        calc_fuel_for_fuel(fuel)
    }).sum::<i32>()
}
