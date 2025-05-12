mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_operations(nums: &[i32], target: &[i32]) -> i64 {
    let mut res = 0;
    let mut prev_sign = 0;
    let mut prev_abs = 0;
    for delta in nums
        .iter()
        .zip(target.iter())
        .map(|(a, b)| i64::from(a - b))
    {
        if prev_sign * delta < 0 {
            prev_abs = 0; // end greedy streak
        }
        res += (delta.abs() - prev_abs).max(0);
        prev_abs = delta.abs();
        prev_sign = delta.signum();
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
        assert_eq!(minimum_operations(&[3, 5, 1, 2], &[4, 6, 2, 4]), 2);
        assert_eq!(minimum_operations(&[1, 3, 2], &[2, 1, 4]), 5);
    }

    #[test]
    fn test() {}
}
