mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_arithmetic_slices(nums: &[i32]) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    let deltas: Vec<_> = nums.windows(2).map(|w| w[0] - w[1]).collect();
    let n = deltas.len();
    let mut dp = vec![0; n];
    for i in 1..n {
        if deltas[i] == deltas[i - 1] {
            dp[i] = dp[i - 1] + 1
        }
    }
    dp.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::DerefMut};

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(number_of_arithmetic_slices(&[1, 2, 3, 4]), 3);
        debug_assert_eq!(number_of_arithmetic_slices(&[1, 2, 3, 4, 5]), 6);
        debug_assert_eq!(number_of_arithmetic_slices(&[1]), 0);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: DerefMut<Target = [T1]>,
        I2: DerefMut<Target = [T2]>,
    {
        i1.sort_unstable();
        i2.sort_unstable();
        debug_assert_eq!(*i1, *i2);
    }
}
