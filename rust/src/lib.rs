mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good_rotations(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let sum: i64 = nums.iter().map(|&v| i64::from(v)).sum();
    nums.extend_from_within(..n / 2 - 1);
    let mut res = 0;
    let mut half = 0;
    for (i, &num) in nums.iter().enumerate() {
        half += i64::from(num);
        if i >= n / 2 {
            half -= i64::from(nums[i - n / 2]);
        }
        if i >= n / 2 - 1 {
            res += i32::from(2 * half > sum)
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(count_good_rotations(vec![1, 2, 3, 4, 5, 6]), 3)
    }

    #[test]
    fn test() {
        assert_eq!(count_good_rotations(vec![10, 6]), 1);
    }
}
