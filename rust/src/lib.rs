mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_number_of_lis(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];
    let mut counts = vec![1; n];
    // for a pentential LIS ending at i
    for i in 1..n {
        // scan all nums [0..i]
        for j in 0..i {
            if nums[i] > nums[j] {
                let curr = 1 + dp[j];
                if curr > dp[i] {
                    // longer! update as 1+dp[j]
                    dp[i] = curr;
                    // number of LIS ending at i is the same as at j
                    counts[i] = counts[j];
                } else if curr == dp[i] {
                    // hits the same length again!
                    // += counts at j
                    counts[i] += counts[j];
                }
            }
        }
    }
    let lis = dp.iter().copied().max().unwrap_or(1);
    // if a LIS ends at i, include its count
    dp.into_iter()
        .zip(counts)
        .filter_map(|(len, count)| if len == lis { Some(count) } else { None })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_number_of_lis(&[1, 3, 5, 4, 7]), 2);
        debug_assert_eq!(find_number_of_lis(&[2, 2, 2, 2, 2]), 5);
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
