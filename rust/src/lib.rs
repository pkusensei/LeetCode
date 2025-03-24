mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 {
    let [a, b, c1, c2] = [divisor1, divisor2, unique_cnt1, unique_cnt2].map(i64::from);
    let _lcm = lcm(a, b);
    let mut left = 1;
    let mut right = i64::from(i32::MAX);
    while left < right {
        let mid = left + (right - left) / 2;
        if mid - mid / a >= c1 && mid - mid / b >= c2 && mid - mid / _lcm >= c1 + c2 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as _
}

const fn lcm(a: i64, b: i64) -> i64 {
    const fn gcd(a: i64, b: i64) -> i64 {
        if a == 0 { b } else { gcd(b % a, a) }
    }
    a / gcd(a, b) * b
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
        assert_eq!(minimize_set(2, 7, 1, 3), 4);
        assert_eq!(minimize_set(3, 5, 2, 1), 3);
        assert_eq!(minimize_set(2, 4, 8, 2), 15);
    }

    #[test]
    fn test() {
        assert_eq!(minimize_set(2, 2, 6, 4), 19);
    }
}
