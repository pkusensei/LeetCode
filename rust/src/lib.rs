mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_palindrome_subseq(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    // let mut dp = vec![vec![-1; n]; n];
    // top_down(s, &mut dp, 0, n - 1)

    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for len in 2..=n {
        for i in 0..(n - len + 1) {
            let j = len + i - 1;
            if s[i] == s[j] {
                dp[i][j] = 2 + dp[i + 1][j - 1]
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1])
            }
        }
    }
    dp[0][n - 1]
}

fn top_down(s: &[u8], dp: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
    if i > j {
        return 0; // empty string
    }
    if i == j {
        return 1; // single byte
    }
    if dp[i][j] > -1 {
        return dp[i][j];
    }
    let res = if s[i] == s[j] {
        2 + top_down(s, dp, i + 1, j - 1)
    } else {
        // skip one byte, either [i] or [j]
        top_down(s, dp, i + 1, j).max(top_down(s, dp, i, j - 1))
    };
    dp[i][j] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_palindrome_subseq("bbbab"), 4);
        debug_assert_eq!(longest_palindrome_subseq("cbbd"), 2);
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
