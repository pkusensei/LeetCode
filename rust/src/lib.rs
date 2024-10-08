mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn strange_printer(s: String) -> i32 {
    let mut s = s.into_bytes();
    s.dedup();
    let n = s.len();
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for length in 2..=n {
        for left in 0..n - length + 1 {
            let right = left + length - 1;
            dp[left][right] = length;
            for split in 0..length - 1 {
                let mut turns = dp[left][left + split] + dp[left + split + 1][right];
                if s[left + split] == s[right] {
                    turns -= 1;
                }
                dp[left][right] = dp[left][right].min(turns);
            }
        }
    }
    dp[0][n - 1] as _
    // solve(&s, &mut dp, 0, n - 1)
}

fn solve(s: &[u8], dp: &mut [Vec<i32>], left: usize, right: usize) -> i32 {
    if left > right {
        return 0;
    }
    if dp[left][right] > 0 {
        return dp[left][right];
    }
    // [a,b,a,..]
    // remove [a]; recurse on [b,a,..]
    let mut res = 1 + solve(s, dp, left + 1, right);
    for i in left + 1..=right {
        if s[left] == s[i] {
            // split into [b] and [a]+[a,..]
            // the latter is the same as [a,..], thus [b] and [a,..]
            // another split is [a,b] and [,..]
            // since 2 a's together can be viewed as one
            res = res.min(solve(s, dp, left + 1, i - 1) + solve(s, dp, i, right))
        }
    }
    dp[left][right] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(strange_printer("aaabbb".into()), 2);
        debug_assert_eq!(strange_printer("aba".into()), 2);
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
