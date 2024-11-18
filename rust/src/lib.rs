mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_sum_after_k_negations(nums: &mut [i32], mut k: i32) -> i32 {
    nums.sort_unstable();
    for num in nums.iter_mut() {
        if *num < 0 && k > 0 {
            *num = -*num;
            k -= 1;
        }
    }
    if k > 0 {
        nums.sort_unstable();
        if k & 1 == 1 {
            nums[0] = -nums[0];
        }
    }
    nums.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(largest_sum_after_k_negations(&mut [4, 2, 3], 1), 5);
        debug_assert_eq!(largest_sum_after_k_negations(&mut [3, -1, 0, 2], 3), 6);
        debug_assert_eq!(
            largest_sum_after_k_negations(&mut [2, -3, -1, 5, -4], 2),
            13
        );
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
