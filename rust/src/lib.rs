mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subarray_sum(nums: &[i32], k: i32) -> i32 {
    let mut prefix = std::collections::HashMap::new();
    prefix.insert(0, 1);
    let mut res = 0;
    let mut sum = 0;
    for &num in nums.iter() {
        sum += num;
        if let Some(&v) = prefix.get(&(sum - k)) {
            res += v;
        }
        *prefix.entry(sum).or_insert(0) += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(subarray_sum(&[1, 1, 1], 2), 2);
        debug_assert_eq!(subarray_sum(&[1, 2, 3], 3), 2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(subarray_sum(&[1, -1, 0], 0), 3);
        debug_assert_eq!(subarray_sum(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), 55);
    }

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
