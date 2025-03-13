mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_numbers(num: i32, k: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    if k == 0 {
        return if num % 10 == 0 { 1 } else { -1 };
    }
    solve(num, k).unwrap_or(-1)
}

fn solve(num: i32, k: i32) -> Option<i32> {
    if num % 10 == k {
        return Some(1);
    }
    if num < k {
        return None;
    }
    solve(num - k, k).map(|v| 1 + v)
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
        assert_eq!(minimum_numbers(58, 9), 2);
        assert_eq!(minimum_numbers(37, 2), -1);
        assert_eq!(minimum_numbers(0, 7), 0);
    }

    #[test]
    fn test() {}
}
