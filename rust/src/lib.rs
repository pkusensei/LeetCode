mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_palindrome(word1: &str, word2: &str) -> i32 {
    let n1 = word1.len();
    let s = format!("{}{}", word1, word2).into_bytes();
    let n = s.len();
    let mut dp = vec![vec![0; n]; n];
    for (i, v) in dp.iter_mut().enumerate() {
        v[i] = 1;
    }
    let mut res = 0;
    for left in (0..n - 1).rev() {
        for right in 1 + left..n {
            if s[left] == s[right] {
                dp[left][right] = 2 + dp[1 + left][right - 1];
                if left < n1 && right >= n1 {
                    res = res.max(dp[left][right]);
                }
            } else {
                dp[left][right] = dp[1 + left][right].max(dp[left][right - 1]);
            }
        }
    }
    res
    // dfs(&s, n1, 0, s.len() - 1, false, &mut HashMap::new())
}

fn dfs(
    s: &[u8],
    n1: usize,
    left: usize,
    right: usize,
    matched: bool,
    memo: &mut HashMap<(usize, usize, bool), i32>,
) -> i32 {
    let k = (left, right, matched);
    if let Some(&v) = memo.get(&k) {
        return v;
    }
    let res = if !matched && (left >= n1 || right < n1) {
        0
    } else if left >= right {
        i32::from(left == right && matched)
    } else if s[left] == s[right] && (matched || (left..=right).contains(&n1)) {
        2 + dfs(s, n1, 1 + left, right - 1, true, memo)
    } else {
        dfs(s, n1, 1 + left, right, matched, memo).max(dfs(s, n1, left, right - 1, matched, memo))
    };
    memo.insert(k, res);
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(longest_palindrome("cacb".into(), "cbba".into()), 5);
        assert_eq!(longest_palindrome("ab".into(), "ab".into()), 3);
        assert_eq!(longest_palindrome("aa".into(), "bb".into()), 0);
    }

    #[test]
    fn test() {
        assert_eq!(longest_palindrome("afaaadacb", "ca"), 6);
    }
}
