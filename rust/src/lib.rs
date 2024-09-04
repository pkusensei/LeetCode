mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn length_of_lis(nums: &[i32]) -> i32 {
    // dp(nums)
    binary_search(nums)
}

fn dp(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];
    for i in 0..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    dp.into_iter().max().unwrap()
}

fn binary_search(nums: &[i32]) -> i32 {
    let mut values = vec![nums[0]];
    for &num in nums.iter() {
        if num > *values.last().unwrap() {
            values.push(num)
        } else if let Err(idx) = values.binary_search(&num) {
            values[idx] = values[idx].min(num)
        }
    }
    values.len() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(length_of_lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
        debug_assert_eq!(length_of_lis(&[0, 1, 0, 3, 2, 3]), 4);
        debug_assert_eq!(length_of_lis(&[7, 7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(length_of_lis(&[4, 10, 4, 3, 8, 9]), 3);
    }

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
