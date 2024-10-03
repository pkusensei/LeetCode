mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn triangle_number(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    if n < 3 {
        return 0;
    }
    nums.sort_unstable();
    let mut res = 0;
    for i in 0..n - 2 {
        let mut left = i + 2;
        if nums[i] == 0 {
            continue;
        }
        for j in i + 1..n - 1 {
            left = binary_searct(nums, left, nums.len() - 1, nums[i] + nums[j]);
            res += (left - j - 1) as i32;
        }
    }
    res
}

fn binary_searct(nums: &[i32], mut left: usize, mut right: usize, target: i32) -> usize {
    while left <= right && right < nums.len() {
        let mid = left + (right - left) / 2;
        if nums[mid] >= target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(triangle_number(&mut [2, 2, 3, 4]), 3);
        debug_assert_eq!(triangle_number(&mut [4, 2, 3, 4]), 4);
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
