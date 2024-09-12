mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn get_money_amount(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0; n + 1]; n + 1];
    solve(&mut dp, 1, n)
}

fn solve(dp: &mut [Vec<i32>], start: usize, end: usize) -> i32 {
    if start >= end {
        return 0;
    }
    if dp[start][end] != 0 {
        return dp[start][end];
    }
    let mut res = i32::MAX;
    for i in start..=end {
        // for each choice i, find worst outcome => i + max(..i-1, i+1..)
        // but with strategy, find best of worst outcomes => min(all i)
        res = res.min(i as i32 + solve(dp, start, i - 1).max(solve(dp, i + 1, end)));
    }
    dp[start][end] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(get_money_amount(10), 16);
        debug_assert_eq!(get_money_amount(1), 0);
        debug_assert_eq!(get_money_amount(2), 1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
