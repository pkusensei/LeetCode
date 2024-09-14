mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_rotate_function(nums: &[i32]) -> i32 {
    let n = nums.len();
    let sum: i32 = nums.iter().sum();
    let mut curr: i32 = nums.iter().enumerate().map(|(i, &n)| (i as i32) * n).sum();
    let mut res = curr;
    for k in 1..nums.len() {
        let i = n - k;
        curr = curr + sum - nums[i] * (n as i32);
        res = res.max(curr);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_rotate_function(&[4, 3, 2, 6]), 26);
        debug_assert_eq!(max_rotate_function(&[100]), 0);
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
