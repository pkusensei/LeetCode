mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32 {
    let (s1, n1) = (source.as_bytes(), source.len());
    let (s2, n2) = (pattern.as_bytes(), pattern.len());
    // let target: HashSet<_> = target_indices.into_iter().map(|v| v as usize).collect();
    // dfs(s1, s2, &target, 0, 0, &mut vec![vec![-1; n2]; n1])
    let mut target = vec![0; n1];
    for i in target_indices {
        target[i as usize] += 1;
    }
    let mut dp = vec![i32::MIN; 1 + n2];
    dp[n2] = 0;
    for i1 in (0..n1).rev() {
        for i2 in 0..=n2 {
            dp[i2] += target[i1];
            if i2 < n2 && s1[i1] == s2[i2] {
                dp[i2] = dp[i2].max(dp[1 + i2]);
            }
        }
    }
    dp[0]
}

// tle's
fn dfs(
    s1: &[u8],
    s2: &[u8],
    target: &HashSet<usize>,
    i1: usize,
    i2: usize,
    memo: &mut [Vec<i32>],
) -> i32 {
    if memo[i1][i2] > -1 {}
    if i1 >= s1.len() {
        return if i2 >= s2.len() { 0 } else { i32::MIN };
    }
    if i2 >= s2.len() {
        return i32::from(target.contains(&i1)) + dfs(s1, s2, target, 1 + i1, i2, memo);
    }
    if memo[i1][i2] > -1 {
        return memo[i1][i2];
    }
    let mut res = i32::from(target.contains(&i1)) + dfs(s1, s2, target, 1 + i1, i2, memo); // skip 
    if s1[i1] == s2[i2] {
        res = res.max(dfs(s1, s2, target, 1 + i1, 1 + i2, memo));
    }
    memo[i1][i2] = res;
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
    fn basics() {}

    #[test]
    fn test() {}
}
