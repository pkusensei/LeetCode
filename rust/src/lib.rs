mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn peak_valley_difference(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut peak = 0;
    let mut valley = 0;
    for i in 1..n - 1 {
        if nums[i - 1] < nums[i] && nums[i] > nums[1 + i] {
            peak += nums[i]
        }
        if nums[i - 1] > nums[i] && nums[i] < nums[1 + i] {
            valley += nums[i];
        }
    }
    peak - valley
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
    fn basics() {}

    #[test]
    fn test() {}
}
