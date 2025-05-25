mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn possible_string_count(word: &str, k: i32) -> i32 {
    const M: i64 = 1_000_000_007;
    let mut nums = vec![];
    let mut prod = 1;
    for len in word
        .as_bytes()
        .chunk_by(|a, b| a == b)
        .map(|ch| ch.len() as i64)
    {
        nums.push(len);
        prod = prod * len % M;
    }
    let (n, k) = (nums.len(), k as usize);
    if k <= n {
        return prod as i32;
    }
    let mut dp = vec![0; k];
    dp[0] = 1;
    for idx in 1..=n {
        let prefix = dp.into_iter().fold(vec![0], |mut acc, v| {
            acc.push((v + acc.last().unwrap_or(&0)) % M);
            acc
        });
        dp = vec![0; k];
        for len in idx..k {
            let prev = len - (nums[idx - 1] as usize).min(len + 1 - idx);
            dp[len] = (prefix[len] - prefix[prev]).rem_euclid(M);
        }
    }
    let less_than_k = dp.iter().fold(0, |acc, v| (acc + v) % M);
    (prod - less_than_k).rem_euclid(M) as i32
}

// fn dfs(nums: &[usize], k: usize, idx: usize, memo: &mut [Vec<Option<usize>>]) -> usize {
//     if k == 0 {
//         return nums[idx..].iter().fold(1, |acc, v| acc * v % M);
//     }
//     if idx >= nums.len() {
//         return 0;
//     }
//     if let Some(v) = memo[idx][k] {
//         return v;
//     }
//     let mut res = 0;
//     for curr in 1..=nums[idx] {
//         res += dfs(nums, k.saturating_sub(curr), 1 + idx, memo);
//         res %= M
//     }
//     memo[idx][k] = Some(res);
//     res
// }

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
        assert_eq!(possible_string_count("aabbccdd", 7), 5);
        assert_eq!(possible_string_count("aabbccdd", 8), 1);
        assert_eq!(possible_string_count("aaabbb", 3), 8);
    }

    #[test]
    fn test() {}
}
