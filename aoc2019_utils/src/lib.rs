pub mod point_2d;

use std::fs;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(6), 720);
    }
}
