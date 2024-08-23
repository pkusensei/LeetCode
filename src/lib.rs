mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn fraction_addition(expression: &str) -> String {
    let s = expression.as_bytes();
    let (mut num, mut denom) = (0, 1);
    let mut idx = 0;
    while idx < s.len() {
        let (mut curr_num, mut curr_denom) = (0, 0);
        let mut is_negative = false;
        if s[idx] == b'+' {
            idx += 1
        }
        if s[idx] == b'-' {
            is_negative = true;
            idx += 1
        }
        while s[idx].is_ascii_digit() {
            let v = (s[idx] - b'0') as i32;
            curr_num = 10 * curr_num + v;
            idx += 1;
        }
        if is_negative {
            curr_num *= -1
        }
        idx += 1;
        while idx < s.len() && s[idx].is_ascii_digit() {
            let v = (s[idx] - b'0') as i32;
            curr_denom = 10 * curr_denom + v;
            idx += 1
        }
        num = num * curr_denom + curr_num * denom;
        denom *= curr_denom;
    }

    let gcd = gcd(num, denom).abs();
    num /= gcd;
    denom /= gcd;
    format!("{num}/{denom}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(fraction_addition("-1/2+1/2"), "0/1");
        debug_assert_eq!(fraction_addition("-1/2+1/2+1/3"), "1/3");
        debug_assert_eq!(fraction_addition("1/3-1/2"), "-1/6");
    }

    #[test]
    fn test() {}
}
