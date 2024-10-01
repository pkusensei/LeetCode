mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_distance(word1: &str, word2: &str) -> i32 {
    if word1.is_empty() {
        return word2.len() as _;
    }
    if word2.is_empty() {
        return word1.len() as _;
    }
    if word1 == word2 {
        return 0;
    }
    let (row, col) = (word1.len(), word2.len());
    let (s1, s2) = (word1.as_bytes(), word2.as_bytes());
    let mut dp = vec![vec![0; 1 + col]; 1 + row];
    for (i, v) in dp[0].iter_mut().enumerate() {
        *v = i as i32;
    }
    for (i, v) in dp.iter_mut().enumerate() {
        v[0] = i as i32;
    }
    for i1 in 1..=row {
        for i2 in 1..=col {
            if s1[i1 - 1] == s2[i2 - 1] {
                dp[i1][i2] = dp[i1 - 1][i2 - 1];
            } else {
                dp[i1][i2] = 1 + dp[i1 - 1][i2].min(dp[i1][i2 - 1])
            }
        }
    }
    dp[row][col]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_distance("sea", "eat"), 2);
        debug_assert_eq!(min_distance("leetcode", "etco"), 4);
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
