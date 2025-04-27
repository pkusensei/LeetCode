mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_power(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    dfs(nums, k, 0, &mut vec![vec![-1; 1 + k as usize]; n]) as i32
}

const M: i64 = 1_000_000_007;

fn dfs(nums: &[i32], k: i32, idx: usize, memo: &mut [Vec<i64>]) -> i64 {
    if k == 0 {
        return mod_pow(2, (nums.len() - idx) as _, M);
    }
    if k < 0 || idx >= nums.len() {
        return 0;
    }
    if memo[idx][k as usize] > -1 {
        return memo[idx][k as usize];
    }
    let skip = dfs(nums, k, 1 + idx, memo);
    let take = dfs(nums, k - nums[idx], 1 + idx, memo);
    memo[idx][k as usize] = (take + 2 * skip) % M;
    memo[idx][k as usize]
}

const fn mod_pow(base: i64, exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(base * base % m, exp >> 1, m)
    } else {
        mod_pow(base * base % m, exp >> 1, m) * base % m
    }
}

pub fn bottom_up(nums: &[i32], k: i32) -> i32 {
    let mut dp = vec![0; 1 + k as usize];
    dp[0] = 1;
    for &num in nums {
        for i in (0..=k).rev() {
            if i < num {
                dp[i as usize] = 2 * dp[i as usize] % M;
            } else {
                dp[i as usize] = (2 * dp[i as usize] + dp[(i - num) as usize]) % M;
            }
        }
    }
    dp[k as usize] as i32
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
        assert_eq!(sum_of_power(&[1, 2, 3], 3), 6);
        assert_eq!(sum_of_power(&[2, 3, 3], 5), 4);
        assert_eq!(sum_of_power(&[1, 2, 3], 7), 0);

        assert_eq!(bottom_up(&[1, 2, 3], 3), 6);
        assert_eq!(bottom_up(&[2, 3, 3], 5), 4);
        assert_eq!(bottom_up(&[1, 2, 3], 7), 0);
    }

    #[test]
    fn test() {}
}
