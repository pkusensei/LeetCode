mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn coin_change(coins: &[i32], amount: i32) -> i32 {
    let n = amount as usize;
    let mut dp = vec![None; 1 + n];
    dp[n] = Some(0);
    for idx in (0..n + 1).rev() {
        if dp[idx].is_none() {
            continue;
        }
        for i in coins.iter().filter_map(|&c| idx.checked_sub(c as usize)) {
            if dp[i].is_some() {
                dp[i] = dp[idx].map(|prev| prev + 1).min(dp[i])
            } else {
                dp[i] = dp[idx].map(|v| v + 1);
            }
        }
    }
    dp[0].unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(coin_change(&[1, 2, 5], 11), 3);
        debug_assert_eq!(coin_change(&[2], 3), -1);
        debug_assert_eq!(coin_change(&[1], 0), 0);
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
