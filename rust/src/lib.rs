mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_stickers(stickers: &[&str], target: &str) -> i32 {
    let n = target.len();
    // Each letter has 2 states => 2^n
    let mut dp = vec![-1; 1 << n];
    dp[0] = 0; // no sticker needed
    for mask in 0..1 << n {
        if dp[mask] == -1 {
            // This mask is not reached in previous loop(s)
            // i.e not possible to build from stickers
            continue;
        }
        for stick in stickers.iter() {
            let mut curr = mask;
            for b1 in stick.bytes() {
                for (idx, b2) in target.bytes().enumerate() {
                    if (curr >> idx) & 1 == 0 && b2 == b1 {
                        curr |= 1 << idx;
                        // use current b1 and break into next loop
                        break;
                    }
                }
            }
            // curr is reachable; record min value
            if dp[curr] == -1 || dp[curr] > dp[mask] + 1 {
                dp[curr] = 1 + dp[mask];
            }
        }
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_stickers(&["with", "example", "science"], "thehat"), 3);
        debug_assert_eq!(min_stickers(&["notice", "possible"], "basicbasic"), -1);
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
