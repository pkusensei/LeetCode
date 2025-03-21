mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
    let mut res = 0;
    let mut num = n;
    while digit_sum(num) > i64::from(target) {
        let d = alter(num);
        res += d;
        num += d;
    }
    res
}

const fn alter(mut num: i64) -> i64 {
    let mut pow = 1;
    while num % 10 == 0 {
        pow *= 10;
        num /= 10;
    }
    pow * (10 - num % 10)
}

const fn digit_sum(mut num: i64) -> i64 {
    let mut res = 0;
    while num > 0 {
        res += num % 10;
        num /= 10;
    }
    res
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
        assert_eq!(make_integer_beautiful(16, 6), 4);
        assert_eq!(make_integer_beautiful(467, 6), 33);
    }

    #[test]
    fn test() {}
}
