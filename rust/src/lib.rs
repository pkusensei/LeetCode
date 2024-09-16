mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn can_partition(nums: &[i32]) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum & 1 == 1 {
        return false;
    }
    let n = nums.len();
    let half = (sum / 2) as usize;
    let mut dp = vec![vec![false; half + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = true // empty subset sums to zero
    }
    for i in 1..=n {
        for j in 1..=half {
            if nums[i - 1] as usize <= j {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i - 1] as usize];
            } else {
                dp[i][j] = dp[i - 1][j]
            }
        }
        if dp[i][half] {
            return true;
        }
    }
    dp[n][half]
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::DerefMut};

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_partition(&[1, 5, 11, 5]));
        debug_assert!(!can_partition(&[1, 2, 3, 5]))
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
