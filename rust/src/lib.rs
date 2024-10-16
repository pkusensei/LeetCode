mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_palindromic_subsequences(s: &str) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let (s, n) = (s.as_bytes(), s.len());
    let mut dp = vec![vec![0i32; n]; n];
    for (i, v) in dp.iter_mut().enumerate() {
        v[i] = 1;
    }
    for i1 in (0..n).rev() {
        for i2 in 1 + i1..n {
            if s[i1] != s[i2] {
                dp[i1][i2] = (dp[i1 + 1][i2] + dp[i1][i2 - 1] - dp[i1 + 1][i2 - 1]).rem_euclid(MOD);
            } else {
                let (mut left, mut right) = (i1 + 1, i2 - 1);
                while left <= right && s[left] != s[i1] {
                    left += 1;
                }
                while left <= right && s[right] != s[i2] {
                    right -= 1
                }
                match left.cmp(&right) {
                    std::cmp::Ordering::Less => {
                        // For "aabaa", or "a..a..a..a"
                        // dp[i1 + 1][i2 - 1] => "a", "b", "aba", "aa"
                        // 2* => Expand to be "aaa", "aba", "aabaa", "aaaa"
                        // remove dup => "aba"
                        dp[i1][i2] =
                            (2 * dp[i1 + 1][i2 - 1] - dp[left + 1][right - 1]).rem_euclid(MOD)
                    }
                    std::cmp::Ordering::Equal => {
                        // For "aaa", or more generally "a..a..a"
                        // 2* => "a" and "aaa"
                        // 1+ => "aa"
                        dp[i1][i2] = (1 + 2 * dp[i1 + 1][i2 - 1]).rem_euclid(MOD);
                    }
                    std::cmp::Ordering::Greater => {
                        // For "aba"
                        // 2*dp[i1+1][i2-1] => center "b" as 1, "aba" as 1
                        // 2+ to add in "a" "aa"
                        dp[i1][i2] = (2 + 2 * dp[i1 + 1][i2 - 1]).rem_euclid(MOD);
                    }
                }
            }
        }
    }
    dp[0][n - 1]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_palindromic_subsequences("bccb"), 6);
        debug_assert_eq!(
            count_palindromic_subsequences(
                "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba"
            ),
            104860361
        );
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
