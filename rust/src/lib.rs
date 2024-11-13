mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_fair_pairs(nums: &mut [i32], lower: i32, upper: i32) -> i64 {
    nums.sort_unstable();
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let left = nums[1 + idx..].partition_point(|&v| v < lower - num);
        let right = nums[1 + idx..].partition_point(|&v| v <= upper - num);
        res += right - left;
    }
    res as i64
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_fair_pairs(&mut [0, 1, 7, 4, 4, 5], 3, 6), 6);
        debug_assert_eq!(count_fair_pairs(&mut [1, 7, 9, 2, 5], 11, 11), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(count_fair_pairs(&mut [0, 0, 0, 0, 0, 0], 0, 0), 15);
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
