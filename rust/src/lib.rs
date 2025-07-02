mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_xor(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    dfs(&nums, k, 0, &mut vec![vec![-1; n]; 1 + k])
}

fn dfs(nums: &[i32], k: usize, idx: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = nums.len();
    if idx >= n {
        return if k == 0 { 0 } else { i32::MAX };
    }
    if k == 0 {
        return i32::MAX;
    }
    if memo[k][idx] > -1 {
        return memo[k][idx];
    }
    let mut xor = 0;
    let mut res = i32::MAX;
    for right in idx..n {
        xor ^= nums[right];
        let val = dfs(nums, k - 1, 1 + right, memo);
        if val < i32::MAX {
            res = res.min(xor.max(val));
        }
    }
    memo[k][idx] = res;
    res
}

pub fn bottom_up(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let prefix = nums.iter().fold(vec![0], |mut acc, v| {
        acc.push(v ^ acc.last().unwrap_or(&0));
        acc
    });
    let mut dp = vec![vec![i32::MAX; n + 1]; k + 1];
    for i in 1..=n {
        dp[1][i] = prefix[i];
    }
    for split in 2..=k {
        for right in split..=n {
            let mut curr = i32::MAX;
            for left in (split - 1)..right {
                let val = dp[split - 1][left].max(prefix[right] ^ prefix[left]);
                curr = curr.min(val);
            }
            dp[split][right] = curr;
        }
    }
    dp[k][n]
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
    fn basics() {}

    #[test]
    fn test() {}
}
