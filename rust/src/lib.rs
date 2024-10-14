mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_delete_sum(s1: &str, s2: &str) -> i32 {
    let (s1, s2, n1, n2) = (s1.as_bytes(), s2.as_bytes(), s1.len(), s2.len());
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for i1 in 1..1 + n1 {
        for i2 in 1..1 + n2 {
            if s1[i1 - 1] == s2[i2 - 1] {
                dp[i1][i2] = i32::from(s1[i1 - 1]) + dp[i1 - 1][i2 - 1];
            } else {
                dp[i1][i2] = dp[i1][i2 - 1].max(dp[i1 - 1][i2]);
            }
        }
    }
    s1.iter().chain(s2).map(|&b| i32::from(b)).sum::<i32>() - 2 * dp[n1][n2]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(minimum_delete_sum("sea", "eat"), 231);
        debug_assert_eq!(minimum_delete_sum("delete", "leet"), 403);
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
