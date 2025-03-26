mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn put_marbles(weights: &[i32], k: i32) -> i64 {
    use itertools::Itertools;
    let n = weights.len();
    let k = k as usize;
    if n == k || k < 2 {
        return 0;
    }
    let mut nums = weights
        .windows(2)
        .map(|w| i64::from(w[0] + w[1]))
        .collect_vec();
    nums.sort_unstable();
    let min: i64 = nums[..k - 1].iter().sum();
    let max: i64 = nums.iter().tail(k - 1).sum();
    max - min
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
        assert_eq!(put_marbles(&[1, 3, 5, 1], 2), 4);
        assert_eq!(put_marbles(&[1, 3], 2), 0);
    }

    #[test]
    fn test() {
        assert_eq!(put_marbles(&[1, 4, 2, 5, 2], 3), 3);
    }
}
