mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn dominant_index(nums: &[i32]) -> i32 {
    let Some((idx, &num)) = nums.iter().enumerate().max_by_key(|(_, &n)| n) else {
        return -1;
    };
    if nums.iter().all(|&n| n == num || 2 * n <= num) {
        idx as _
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(dominant_index(&[3, 6, 1, 0]), 1);
        debug_assert_eq!(dominant_index(&[1, 2, 3, 4]), -1);
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
