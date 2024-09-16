mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn split_array(nums: &[i32], k: i32) -> i32 {
    let mut low = *nums.iter().max().unwrap();
    let mut high = nums.iter().sum();
    // 1) the largest sum of any subarray lies in max..=sum
    // 2) the seq max..=sum is sorted
    while low <= high {
        // Attempt mid
        let mid = low + (high - low) / 2;
        // it produces count number of subarrays
        let count = count_splits(nums, mid);
        if count > k {
            low = mid + 1
        } else {
            high = mid - 1
        }
    }
    low
}

fn count_splits(nums: &[i32], upper: i32) -> i32 {
    let mut res = 1;
    let mut sum = 0;
    for &num in nums.iter() {
        if sum + num <= upper {
            sum += num
        } else {
            res += 1;
            sum = num;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::DerefMut};

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(split_array(&[7, 2, 5, 10, 8], 2), 18);
        debug_assert_eq!(split_array(&[1, 2, 3, 4, 5], 2), 9);
    }

    #[test]
    fn test() {
        debug_assert_eq!(split_array(&[1, 4, 4], 3), 4);
        debug_assert_eq!(split_array(&[2, 3, 1, 2, 4, 3], 5), 4);
    }

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
