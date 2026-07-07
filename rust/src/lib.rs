mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_dp(word1: &str, word2: &str, target: &str) -> i32 {
    let [n1, n2, n3] = [&word1, &word2, &target].map(|s| s.len());
    let [s1, s2, target] = [&word1, &word2, &target].map(|s| s.as_bytes());
    let mut dp = vec![vec![vec![0; 1 + n3]; 1 + n2]; 1 + n1];
    for i1 in 1..=n1 {
        for i2 in 1..=n2 {
            dp[i1][i2][n3] = 1;
        }
    }
    for i3 in (0..n3).rev() {
        for i1 in (0..=n1).rev() {
            for i2 in (0..=n2).rev() {
                for ii in i1..n1 {
                    if s1[ii] == target[i3] {
                        dp[i1][i2][i3] = (dp[i1][i2][i3] + dp[1 + ii][i2][1 + i3]) % M;
                    }
                }
                for ii in i2..n2 {
                    if s2[ii] == target[i3] {
                        dp[i1][i2][i3] = (dp[i1][i2][i3] + dp[i1][1 + ii][1 + i3]) % M;
                    }
                }
            }
        }
    }
    dp[0][0][0]
}

pub fn interleave_characters(word1: &str, word2: &str, target: &str) -> i32 {
    let [n1, n2, n3] = [&word1, &word2, &target].map(|s| s.len());
    let [s1, s2, target] = [&word1, &word2, &target].map(|s| s.as_bytes());
    let mut memo = vec![vec![vec![-1; n3]; 1 + n2]; 1 + n1];
    dfs(s1, s2, target, 0, 0, 0, &mut memo)
}

const M: i32 = 1_000_000_007;

fn dfs(
    s1: &[u8],
    s2: &[u8],
    target: &[u8],
    i1: usize,
    i2: usize,
    i3: usize,
    memo: &mut [Vec<Vec<i32>>],
) -> i32 {
    if i3 == target.len() {
        return i32::from(i1 > 0 && i2 > 0);
    }
    if i1 >= s1.len() && i2 >= s2.len() {
        return 0;
    }
    if memo[i1][i2][i3] > -1 {
        return memo[i1][i2][i3];
    }
    let mut res = 0;
    for ii in i1..s1.len() {
        if s1[ii] == target[i3] {
            res = (res + dfs(s1, s2, target, 1 + ii, i2, 1 + i3, memo)) % M
        }
    }
    for ii in i2..s2.len() {
        if s2[ii] == target[i3] {
            res = (res + dfs(s1, s2, target, i1, 1 + ii, 1 + i3, memo)) % M;
        }
    }
    memo[i1][i2][i3] = res;
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
    fn basics() {
        assert_eq!(interleave_characters("abc", "bac", "abc"), 5);
        assert_eq!(with_dp("abc", "bac", "abc"), 5);
    }

    #[test]
    fn test() {}
}
