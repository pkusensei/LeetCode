mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_target_sum_ways(nums: &[i32], target: i32) -> i32 {
    // match nums {
    //     [] => (0 == target).into(),
    //     [n, tail @ ..] => {
    //         find_target_sum_ways(tail, target + *n) + find_target_sum_ways(tail, target - *n)
    //     }
    // }
    with_dp(nums, target)
}

fn with_dp(nums: &[i32], target: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    if target.abs() > sum || (target + sum) & 1 > 0 {
        return 0;
    }
    let p = (sum + target) / 2;
    let mut dp = vec![0; 1 + p as usize];
    dp[0] = 1;
    for &num in nums.iter() {
        for i in (num..=p).rev() {
            dp[i as usize] += dp[(i - num) as usize];
        }
    }
    dp[p as usize]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_target_sum_ways(&[1, 1, 1, 1, 1], 3), 5);
        debug_assert_eq!(find_target_sum_ways(&[1], 1), 1);
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
