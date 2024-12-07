mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_size(nums: &[i32], max_operations: i32) -> i32 {
    let mut left = 1;
    let mut right = nums.iter().copied().max().unwrap_or(nums[0]);
    while left < right {
        let mid = left + (right - left) / 2;
        if count(nums, mid) > nums.len() as i32 + max_operations {
            left = 1 + mid;
        } else {
            right = mid;
        }
    }
    left
}

fn count(nums: &[i32], mid: i32) -> i32 {
    nums.iter()
        .map(|num| num / mid + i32::from(num % mid > 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(minimum_size(&[9], 2), 3);
        assert_eq!(minimum_size(&[2, 4, 8, 2], 4), 2);
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
