mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subarray(nums: &[i32]) -> i32 {
    let mut res = 2;
    let n = nums.len();
    let mut left = 0;
    let mut right = 1;
    while 1 + right < n {
        while 1 + right < n
            && i64::from(nums[right - 1]) + i64::from(nums[right]) == i64::from(nums[1 + right])
        {
            right += 1;
        }
        right += 1;
        res = res.max(right - left);
        left = right - 1;
    }
    res as i32
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
        assert_eq!(longest_subarray(&[1, 1, 1, 1, 2, 3, 5, 1]), 5);
        assert_eq!(longest_subarray(&[5, 2, 7, 9, 16]), 5);
        assert_eq!(longest_subarray(&[1000000000, 1000000000, 1000000000]), 2);
    }

    #[test]
    fn test() {}
}
