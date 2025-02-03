mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_partitioning(s: &str) -> bool {
    let n = s.len();
    dfs(s, 0, 2, &mut vec![vec![None; n]; 3])
}

fn dfs(s: &str, idx: usize, count: usize, memo: &mut [Vec<Option<bool>>]) -> bool {
    if count == 0 {
        return idx < s.len() && is_palindrome(s[idx..].bytes());
    }
    if let Some(v) = memo[count][idx] {
        return v;
    }
    let mut res = false;
    for i in 1 + idx..s.len() {
        if is_palindrome(s[idx..i].bytes()) && dfs(s, i, count - 1, memo) {
            res = true;
            break;
        }
    }
    memo[count][idx] = Some(res);
    res
}

pub fn bottom_up(s: &str) -> bool {
    let (s, n) = (s.as_bytes(), s.len());
    let mut dp = vec![vec![false; n]; n];
    for i in 0..n {
        dp[i][i] = true;
    }
    for i in 0..n - 1 {
        dp[i][i + 1] = s[i] == s[i + 1];
    }
    for len in 3..=n {
        for left in 0..n - len {
            let right = left + len - 1;
            dp[left][right] = s[left] == s[right] && dp[1 + left][right - 1];
        }
    }
    for left in 1..n - 1 {
        if dp[0][left - 1] {
            for right in 1 + left..n {
                if dp[left][right - 1] && dp[right][n - 1] {
                    return true;
                }
            }
        }
    }
    false
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
        assert!(check_partitioning("abcbdd"));
        assert!(!check_partitioning("bcbddxy"));

        assert!(bottom_up("abcbdd"));
        assert!(!bottom_up("bcbddxy"));
    }

    #[test]
    fn test() {
        assert!(!check_partitioning("acab"));
        assert!(!bottom_up("acab"));
    }
}
