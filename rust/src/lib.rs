mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn change(amount: i32, coins: &[i32]) -> i32 {
    let n = amount as usize;
    let mut dp = vec![0; 1 + n];
    dp[0] = 1; // empty

    // Think backwards
    // To reach n, for each number < n, e.g a
    // a + delta == n
    // Thus find all a's that are delta away from any n
    // Now do the same for all deltas.
    // This also prevents duplicates.
    // For example with amount == 3 and coins == [1, 2]
    // {1, 2} and {2, 1} are the same
    for &delta in coins {
        for i in 0..n {
            if delta as usize + i <= n {
                dp[delta as usize + i] += dp[i]
            }
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(change(5, &[1, 2, 5]), 4);
        debug_assert_eq!(change(3, &[2]), 0);
        debug_assert_eq!(change(10, &[10]), 1);
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
