mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
    const M: i32 = 1_000_000_007;
    let n = nums.len();
    let max = *nums.iter().max().unwrap_or(&0);
    let mut dp = vec![1; 1 + max as usize];
    // arr2[i-1]>=arr2[i]
    // nums[i-1]-arr1[i-1] >= nums[i]-arr1[i]
    // arr1[i-1] + nums[i]-nums[i-1] <= arr1[i]
    // arr1[i-1] + d <= arr1[i]
    // Everything smaller than val1-d can reach val1
    // curr[val1] = dp[val1-d] + dp[val1-d-1]+..+dp[0]
    // By extension, val1-1 can be reached by...
    // curr[val1-1] = dp[val1-d-1] +..+dp[0]
    // curr[val1]-curr[val-1]=dp[val1-d]
    for idx in 1..n {
        let mut curr = vec![0; 1 + max as usize];
        let diff = (nums[idx] - nums[idx - 1]).max(0);
        for val1 in diff..=nums[idx] {
            curr[val1 as usize] =
                if val1 > 0 { curr[val1 as usize - 1] } else { 0 } + dp[(val1 - diff) as usize];
            curr[val1 as usize] %= M;
        }
        dp = curr;
    }
    dp[..=nums[n - 1] as usize]
        .iter()
        .fold(0, |acc, v| (acc + v) % M)
    // dfs(&nums, 0, 0, i32::MAX, &mut vec![[-1; 51]; n])
}

fn dfs(nums: &[i32], idx: usize, prev1: i32, prev2: i32, memo: &mut [[i32; 51]]) -> i32 {
    if idx >= nums.len() {
        return 1;
    }
    if memo[idx][prev1 as usize] > -1 {
        return memo[idx][prev1 as usize];
    }
    let mut res = 0;
    for val1 in prev1..=nums[idx] {
        let val2 = nums[idx] - val1;
        if val2 <= prev2 {
            res += dfs(nums, 1 + idx, val1, val2, memo);
            res %= 1_000_000_007;
        }
    }
    memo[idx][prev1 as usize] = res;
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
        assert_eq!(count_of_pairs(vec![2, 3, 2]), 4);
        assert_eq!(count_of_pairs(vec![5, 5, 5, 5]), 126);
    }

    #[test]
    fn test() {}
}
