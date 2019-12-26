use std::str::FromStr;

pub fn extract_range(s: &str) -> (u32, u32) {
    let mut parts = s.split("-");
    let num1 = u32::from_str(parts.next().unwrap()).unwrap();
    let num2 = u32::from_str(parts.next().unwrap()).unwrap();
    (num1, num2)
}

fn is_password(num: u32) -> bool {
    let s = num.to_string();
    let bytes = s.as_bytes();
    let mut all_increasing = true;
    let mut any_doubles = false;
    for i in 0..(bytes.len() - 1) {
        let c1 = bytes[i];
        let c2 = bytes[i + 1];
        all_increasing &= c1 <= c2;
        any_doubles |= c1 == c2;
    }

    all_increasing && any_doubles
}

fn is_password_v2(num: u32) -> bool {
    let s = num.to_string();
    let bytes = s.as_bytes();
    let mut all_increasing = true;
    let mut any_doubles = false;
    for i in 0..(bytes.len() - 1) {
        let c1 = bytes[i];
        let c2 = bytes[i + 1];
        
        all_increasing &= c1 <= c2;

        any_doubles |= if c1 == c2 {
            let front_ok = (i == 0) || (c1 != bytes[i - 1]);
            let end_ok = (i == bytes.len() - 2) || (c1 != bytes[i + 2]);
            front_ok && end_ok
        } else {
            false
        }
    }

    all_increasing && any_doubles
}

pub fn count_passwords_in_range(start: u32, end: u32) -> u32 {
    (start..=end).filter(|n| is_password(*n)).count() as u32
}

pub fn count_passwords_in_range_v2(start: u32, end: u32) -> u32 {
    (start..=end).filter(|n| is_password_v2(*n)).count() as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_range() {
        use super::*;

        assert_eq!(extract_range("122345-223450"), (122345,223450));
    }

    #[test]
    fn test_is_password() {
        use super::*;

        assert_eq!(is_password(122345), true);
        assert_eq!(is_password(111123), true);
        assert_eq!(is_password(111111), true);
        assert_eq!(is_password(223450), false);
        assert_eq!(is_password(123789), false);
    }

    #[test]
    fn test_is_password_v2() {
        use super::*;

        assert_eq!(is_password_v2(112233), true);
        assert_eq!(is_password_v2(123444), false);
        assert_eq!(is_password_v2(111122), true);
        assert_eq!(is_password_v2(223450), false);
        assert_eq!(is_password_v2(123789), false);
    }

    #[test]
    fn test_count_passwords_in_range() {
        use super::*;

        assert_eq!(count_passwords_in_range(123, 135), 1);
    }
}
