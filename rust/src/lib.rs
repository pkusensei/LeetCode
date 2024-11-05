mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_range_ii(nums: &mut [i32], k: i32) -> i32 {
    nums.sort_unstable();
    let (first, last) = (nums[0], *nums.last().unwrap());
    let temp = last - first;
    nums.windows(2)
        .map(|w| {
            let min = (first + k).min(w[1] - k);
            let max = (last - k).max(w[0] + k);
            max - min
        })
        .min()
        .map(|v| v.min(temp))
        .unwrap_or(temp)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(smallest_range_ii(&mut [1], 0), 0);
        debug_assert_eq!(smallest_range_ii(&mut [0, 10], 2), 6);
        debug_assert_eq!(smallest_range_ii(&mut [1, 3, 6], 3), 3);
    }

    #[test]
    fn test() {
        debug_assert_eq!(smallest_range_ii(&mut [2, 7, 2], 1), 3);
        debug_assert_eq!(smallest_range_ii(&mut [7, 8, 8], 5), 1);
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
