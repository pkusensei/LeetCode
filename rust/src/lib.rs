mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_valid_string(s: &str) -> bool {
    let (mut stopen, mut ststar) = (vec![], vec![]);
    for (i, b) in s.bytes().enumerate() {
        match b {
            b'(' => stopen.push(i),
            b'*' => ststar.push(i),
            _ => {
                if !stopen.is_empty() {
                    stopen.pop();
                } else if !ststar.is_empty() {
                    ststar.pop();
                } else {
                    return false;
                }
            }
        }
    }
    if stopen.is_empty() {
        true
    } else {
        stopen.len() <= ststar.len()
            && ststar
                .into_iter()
                .rev()
                .zip(stopen.into_iter().rev())
                .all(|(a, b)| a > b)
    }
}

// O(n^2) => TLE
fn with_dp(s: &str) -> bool {
    let (s, n) = (s.as_bytes(), s.len());
    let mut dp = vec![vec![0; n]; n];
    dfs(s, &mut dp, 0, 0) == 2
}

fn dfs(s: &[u8], dp: &mut [Vec<i32>], idx: usize, open: usize) -> i32 {
    if idx == s.len() {
        return if open == 0 { 2 } else { 1 };
    }
    if dp[idx][open] > 0 {
        return dp[idx][open];
    }
    let mut res = 0;
    if s[idx] == b'*' {
        res = res.max(dfs(s, dp, 1 + idx, 1 + open)); // '*' as '('
        if open > 0 {
            res = res.max(dfs(s, dp, 1 + idx, open - 1)); // '*' as ')'
        }
        res = res.max(dfs(s, dp, 1 + idx, open)); // '*' as empty
    } else {
        if s[idx] == b'(' {
            res = dfs(s, dp, 1 + idx, 1 + open);
        } else if open > 0 {
            res = dfs(s, dp, 1 + idx, open - 1);
        }
    }
    dp[idx][open] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(with_dp("()"));
        debug_assert!(with_dp("(*)"));
        debug_assert!(with_dp("(*))"));
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
