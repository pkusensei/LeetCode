mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn can_partition(nums: &[i32]) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum & 1 == 1 {
        return false;
    }
    dfs(nums, 0, sum / 2, &mut HashMap::new())
}

fn dfs(nums: &[i32], idx: usize, target: i32, memo: &mut HashMap<(usize, i32), bool>) -> bool {
    if target == 0 {
        return true;
    }
    if idx >= nums.len() || target < 0 {
        return false;
    }
    if let Some(&v) = memo.get(&(idx, target)) {
        return v;
    }
    let skip = dfs(nums, 1 + idx, target, memo);
    let take = dfs(nums, 1 + idx, target - nums[idx], memo);
    let res = skip || take;
    memo.insert((idx, target), res);
    res
}

pub fn tabulation(nums: &[i32]) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum & 1 == 1 {
        return false;
    }
    let target = (sum / 2) as usize;
    let mut dp = vec![false; 1 + target];
    dp[0] = true;
    for &num in nums {
        let num = num as usize;
        if num > target {
            return false;
        }
        for prev in (0..=target - num).rev() {
            if dp[prev] {
                dp[prev + num] = true;
            }
        }
        if dp[target] {
            return true;
        }
    }
    dp[target]
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
        assert!(can_partition(&[1, 5, 11, 5]));
        assert!(!can_partition(&[1, 2, 3, 5]));

        assert!(tabulation(&[1, 5, 11, 5]));
        assert!(!tabulation(&[1, 2, 3, 5]));
    }

    #[test]
    fn test() {}
}
