mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_number(num: i64) -> i64 {
    let neg = num < 0;
    let mut num = num.abs();
    let mut digits = [0; 10];
    while num > 0 {
        digits[(num % 10) as usize] += 1;
        num /= 10;
    }
    if neg {
        let mut res = 0;
        for (d, mut count) in digits.into_iter().enumerate().rev() {
            while count > 0 {
                res = 10 * res + d as i64;
                count -= 1;
            }
        }
        -res
    } else {
        let Some(first) = digits[1..].iter().position(|&v| v > 0) else {
            return 0;
        };
        digits[1 + first] -= 1;
        let mut res = 1 + first as i64;
        for (d, mut count) in digits.into_iter().enumerate() {
            while count > 0 {
                res = 10 * res + d as i64;
                count -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(smallest_number(310), 103);
        assert_eq!(smallest_number(-7605), -7650);
    }

    #[test]
    fn test() {}
}
