mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    nums.sort_unstable();
    // (index to prev element, count)
    let mut dp: Vec<_> = (0..n).map(|i| (i, 1)).collect();
    for (i, &num) in nums.iter().enumerate() {
        for j in 0..i {
            if num % nums[j] == 0 && dp[j].1 >= dp[i].1 {
                dp[i] = (j, dp[j].1 + 1);
            }
        }
    }
    let (mut idx, mut pair) = dp
        .iter()
        .copied()
        .enumerate()
        .max_by_key(|(_i, pair)| pair.1)
        .unwrap_or((0, dp[0]));
    let mut res = vec![];
    while pair.1 > 1 {
        res.push(nums[idx]);
        idx = pair.0;
        pair = dp[pair.0];
    }
    res.push(nums[idx]);
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(largest_divisible_subset(vec![1, 2, 3]), [1, 3]);
        debug_assert_eq!(largest_divisible_subset(vec![1, 2, 4, 8]), [1, 2, 4, 8]);
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
