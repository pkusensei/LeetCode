mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 1;
    let mut left = 0;
    let mut prev = 0;
    for (right, &num) in nums.iter().enumerate().skip(1) {
        if num > nums[right - 1] {
            continue;
        }
        let a = prev.min(right - left);
        let b = (right - left) / 2;
        prev = right - left;
        res = res.max(a).max(b);
        left = right;
    }
    let a = prev.min(n - left);
    let b = (n - left) / 2;
    res = res.max(a).max(b);
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
        assert_eq!(max_increasing_subarrays(&[2, 5, 7, 8, 9, 2, 3, 4, 3, 1]), 3);
    }

    #[test]
    fn test() {
        assert_eq!(max_increasing_subarrays(&[5, 8, -2, -1]), 2);
    }
}
