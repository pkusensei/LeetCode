mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn best_rotation(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut change = vec![1; n];
    for idx in 0..n {
        // i+n-nums[i] is the moves k to make i==nums[i]
        change[(1 + n + idx - nums[idx] as usize) % n] -= 1
    }
    let mut max_i = 0;
    let mut prefix = 0;
    let mut max = i32::MIN;
    for (idx, num) in change.into_iter().enumerate() {
        prefix += num;
        if prefix > max {
            max = prefix;
            max_i = idx;
        }
    }
    max_i as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(best_rotation(&[2, 3, 1, 4, 0]), 3);
        debug_assert_eq!(best_rotation(&[1, 3, 0, 2, 4]), 0);
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
