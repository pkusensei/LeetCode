mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_combinations(num: &str) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let (n, s) = (num.len(), num.as_bytes());
    if s[0] == b'0' {
        return 0;
    }
    // dfs(num.as_bytes(), 0, 1, &mut vec![vec![-1; n]; n])

    // longest commmon prefix
    let mut lcp = vec![vec![0; 1 + n]; 1 + n];
    for i1 in (0..n).rev() {
        for i2 in (0..n).rev() {
            if s[i1] == s[i2] {
                lcp[i1][i2] = 1 + lcp[1 + i1][1 + i2];
            }
        }
    }
    let mut dp = vec![vec![0; 1 + n]; 1 + n];
    dp[0] = vec![1; 1 + n];
    for idx in 1..n {
        if s[idx] == b'0' {
            continue;
        }
        let mut prev = idx.checked_sub(1);
        let mut sum = 0;
        for right in idx..n {
            dp[idx][right] = sum;
            if let Some(k) = prev {
                if s[k] > b'0' {
                    let len = lcp[k][idx];
                    if len >= right + 1 - idx || s[k + len] < s[idx + len] {
                        dp[idx][right] += dp[k][idx - 1];
                        dp[idx][right] %= MOD;
                    }
                }
                sum += dp[k][idx - 1];
                sum %= MOD;
                prev = k.checked_sub(1);
            }
        }
    }
    dp.into_iter().fold(0, |acc, v| (acc + v[n - 1]) % MOD)
}

// TLE's
fn dfs(s: &[u8], i1: usize, len: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = s.len();
    if i1 >= n {
        return 1;
    }
    if s[i1] == b'0' {
        return 0;
    }
    if memo[i1][len] > -1 {
        return memo[i1][len];
    }
    let mut res = 0;
    for i2 in i1 + len..=n {
        if let Some(prev) = i1.checked_sub(len) {
            if len < (i2 - i1) || s[prev..i1] < s[i1..i2] {
                res += dfs(s, i2, i2 - i1, memo);
            }
        } else {
            res += dfs(s, i2, i2 - i1, memo);
        }
        res %= 1_000_000_007;
    }
    memo[i1][len] = res;
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
        assert_eq!(number_of_combinations("327"), 2);
        assert_eq!(number_of_combinations("094"), 0);
        assert_eq!(number_of_combinations("0"), 0);
    }

    #[test]
    fn test() {}
}
