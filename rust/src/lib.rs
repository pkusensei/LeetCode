mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_subarray_bounded_max(nums: &[i32], left: i32, right: i32) -> i32 {
    let mut res = 0;
    let mut count = 0;
    let mut i1 = 0;
    for (i2, &num) in nums.iter().enumerate() {
        if (left..=right).contains(&num) {
            count = i2 - i1 + 1;
            res += count
        } else if num < left {
            res += count
        } else {
            i1 = 1 + i2;
            count = 0
        }
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_subarray_bounded_max(&[2, 1, 4, 3], 2, 3), 3);
        debug_assert_eq!(num_subarray_bounded_max(&[2, 9, 2, 5, 6], 2, 8), 7);
        debug_assert_eq!(
            num_subarray_bounded_max(&[73, 55, 36, 5, 55, 14, 9, 7, 72, 52], 32, 69),
            22
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            num_subarray_bounded_max(&[45, 2, 49, 6, 55, 73, 27, 17, 4, 71], 17, 33),
            5
        );
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
