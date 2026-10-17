mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::LazyLock;

#[allow(unused_imports)]
use helper::*;

pub fn max_prime_subset(n: i32, s: i32) -> Vec<i32> {
    let i = P.partition_point(|&v| v <= n.min(s));
    let nums = &P[..i];
    let n = i; // nums.len();
    let mut memo = vec![vec![-1; 1 + s as usize]; 1 + n];
    let mut s = s;
    let mut count = i32::MAX >> 1;
    while s >= 0 {
        count = dfs(nums, 0, s, &mut memo);
        if count < i32::MAX >> 1 {
            break;
        }
        s -= 1;
    }
    let mut res = Vec::with_capacity(count as usize);
    for (idx, &num) in nums.iter().enumerate() {
        if s >= num && dfs(nums, 1 + idx, s - num, &mut memo) == count - 1 {
            res.push(num);
            s -= num;
            count -= 1;
        }
    }
    res
}

fn dfs(nums: &[i32], idx: usize, s: i32, memo: &mut [Vec<i32>]) -> i32 {
    if s <= 0 {
        return 0;
    }
    if idx >= nums.len() {
        return i32::MAX >> 1;
    }
    if memo[idx][s as usize] > -1 {
        return memo[idx][s as usize];
    }
    let mut res = dfs(nums, 1 + idx, s, memo);
    if nums[idx] <= s {
        let take = dfs(nums, 1 + idx, s - nums[idx], memo);
        if take < i32::MAX >> 1 {
            res = res.min(1 + take);
        }
    }
    memo[idx][s as usize] = res;
    res
}

const M: usize = 1_000;
static P: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut sieve = [true; 1 + M];
    sieve[..2].fill(false);
    for p in 2..=M.isqrt() {
        if sieve[p] {
            for val in (p * p..=M).step_by(p) {
                sieve[val] = false
            }
        }
    }
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v { Some(i as i32) } else { None })
        .collect()
});

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
