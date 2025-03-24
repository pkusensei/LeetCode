mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_partitions(nums: &[i32], k: i32) -> i32 {
    let k = k as usize;
    let n = nums.len();
    let sum: usize = nums.iter().map(|&v| v as usize).sum();
    if sum < 2 * k {
        return 0;
    }
    let total = (0..n).fold(1, |acc, _| acc * 2 % MOD);
    // let mut memo = vec![vec![-1; k]; 1 + n];
    // let invalid = (0..k).fold(0, |acc, t| (acc + dfs(nums, t, &mut memo)) % MOD);
    let mut dp = vec![0; k];
    dp[0] = 1;
    for &num in nums.iter() {
        let mut target = k - 1;
        while target >= num as usize {
            dp[target] += dp[target - num as usize];
            dp[target] %= MOD;
            target -= 1;
        }
    }
    let invalid = dp.into_iter().fold(0, |acc, v| (acc + v) % MOD);
    (total - 2 * invalid).rem_euclid(MOD)
}

const MOD: i32 = 1_000_000_007;

fn dfs(nums: &[i32], target: usize, memo: &mut [Vec<i32>]) -> i32 {
    if target == 0 {
        return 1;
    }
    match nums {
        [] => 0,
        [head, tail @ ..] => {
            let n = nums.len();
            if memo[n][target] > -1 {
                return memo[n][target];
            }
            let mut res = dfs(tail, target, memo); // skip
            if *head as usize <= target {
                res += dfs(tail, target - *head as usize, memo);
                res %= MOD;
            }
            memo[n][target] = res;
            res
        }
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
        assert_eq!(count_partitions(&[1, 2, 3, 4], 4), 6);
        assert_eq!(count_partitions(&[3, 3, 3], 4), 0);
        assert_eq!(count_partitions(&[6, 6], 4), 2);
    }

    #[test]
    fn test() {}
}
