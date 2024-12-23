mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_divisor(nums: &[i32], threshold: i32) -> i32 {
    let mut left = 1;
    let mut right = nums.iter().copied().max().unwrap_or(i32::MAX);
    while left < right {
        let mid = left + (right - left) / 2;
        let curr: i32 = nums
            .iter()
            .map(|num| num / mid + i32::from(num % mid > 0))
            .sum();
        if curr > threshold {
            left = mid + 1
        } else {
            right = mid
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
        assert_eq!(smallest_divisor(&[1, 2, 5, 9], 6), 5);
        assert_eq!(smallest_divisor(&[44, 22, 33, 11, 1], 5), 44);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
