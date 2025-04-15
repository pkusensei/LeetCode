mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn length_of_longest_subsequence(nums: &[i32], target: i32) -> i32 {
    let n = nums.len();
    let mut memo = vec![vec![None; 1 + target as usize]; n];
    dfs(nums, target, 0, &mut memo).unwrap_or(-1)
}

fn dfs(nums: &[i32], target: i32, idx: usize, memo: &mut [Vec<Option<i32>>]) -> Option<i32> {
    if target <= 0 {
        return if target == 0 { Some(0) } else { None };
    }
    if idx >= nums.len() {
        return None;
    }
    if let Some(v) = memo[idx][target as usize] {
        return if v == -1 { None } else { Some(v) };
    }
    let skip = dfs(nums, target, 1 + idx, memo);
    let take = dfs(nums, target - nums[idx], 1 + idx, memo).map(|v| 1 + v);
    let res = skip.max(take);
    memo[idx][target as usize] = if res.is_none() { Some(-1) } else { res };
    res
}

pub fn tabulation(nums: &[i32], target: i32) -> i32 {
    let mut dp = vec![i32::MIN; 1 + target as usize];
    dp[0] = 0;
    for &num in nums.iter() {
        for i in (num..=target).rev() {
            dp[i as usize] = dp[i as usize].max(1 + dp[(i - num) as usize]);
        }
    }
    if dp[target as usize] < 0 {
        -1
    } else {
        dp[target as usize]
    }
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
        assert_eq!(length_of_longest_subsequence(&[1, 2, 3, 4, 5], 9), 3);
        assert_eq!(length_of_longest_subsequence(&[4, 1, 3, 2, 1, 5], 7), 4);
        assert_eq!(length_of_longest_subsequence(&[1, 1, 5, 4, 5], 3), -1);

        assert_eq!(tabulation(&[1, 2, 3, 4, 5], 9), 3);
        assert_eq!(tabulation(&[4, 1, 3, 2, 1, 5], 7), 4);
        assert_eq!(tabulation(&[1, 1, 5, 4, 5], 3), -1);
    }

    #[test]
    fn test() {}
}
