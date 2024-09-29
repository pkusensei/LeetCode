mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn single_non_duplicate(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n < 2 {
        return nums[0];
    }
    let (mut left, mut right) = (0, n - 1);
    while left < right {
        let mid = left + (right - left) / 2;
        // mid^1  odd  => mid-1
        //        even => mid+1
        if nums.get(mid) == nums.get(mid ^ 1) {
            left = mid + 1;
        } else {
            right = mid;
        }

        // if mid & 1 == 1 {
        //     if nums.get(mid) == nums.get(mid - 1) {
        //         left = mid + 1;
        //     } else {
        //         right = mid - 1;
        //     }
        // } else if nums.get(mid) == nums.get(mid + 1) {
        //     left = mid
        // } else {
        //     right = mid
        // }
    }
    nums[left]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(single_non_duplicate(&[1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
        debug_assert_eq!(single_non_duplicate(&[3, 3, 7, 7, 10, 11, 11]), 10);
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
