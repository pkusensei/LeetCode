mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 3 {
        return -1;
    }
    let max = n + *nums.iter().max().unwrap() as usize;
    let left = dp(&nums, max);
    let rev: Vec<i32> = nums.iter().copied().rev().collect();
    let right = dp(&rev, max);
    let mut res = i32::MAX >> 1;
    for i in 1..n - 1 {
        for v in 1..=max {
            let curr = left[i - 1][v - 1] + right[n - i - 2][v - 1] + (nums[i] - v as i32).abs();
            res = res.min(curr);
        }
    }
    res
}

fn dp(nums: &[i32], max: usize) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut dp = vec![vec![i32::MAX >> 2; 1 + max]; n];
    for (idx, &num) in nums.iter().enumerate() {
        for val in 1..=max {
            let curr = (num - val as i32).abs() + if idx > 0 { dp[idx - 1][val - 1] } else { 0 };
            dp[idx][val] = dp[idx][val - 1].min(curr);
        }
    }
    dp
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
